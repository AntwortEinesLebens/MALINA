// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    errors::{self, Template},
    laboratories::{Identifier, Laboratory, Machine, machines::operating_system::OperatingSystem},
    providers::{Drive, MachineDefinition, ProviderFuture},
    state,
    template::Engine,
};
use std::path::Path;
use tera::Context;
use tokio::task;
use virt::{connect::Connect, domain::Domain, error, network::Network as LibvirtNetwork, sys};

#[derive(Clone, Copy, Debug)]
pub struct Configuration {
    pub provider_identifier: &'static str,
    pub uri: &'static str,
}

#[derive(Debug, Clone)]
pub struct Libvirt {
    configuration: Configuration,
    engine: Engine,
}

impl Libvirt {
    pub fn new(configuration: Configuration) -> Result<Self, Template> {
        let engine = Engine::new()?;

        Ok(Self {
            configuration,
            engine,
        })
    }

    pub fn verify_availability(&self) -> ProviderFuture<()> {
        let provider_identifier = self.configuration.provider_identifier;
        let uri = self.configuration.uri;

        run_blocking(move || {
            let _ = Self::connect(provider_identifier, uri)?;

            Ok(())
        })
    }

    pub fn create_machine(&self, definition: MachineDefinition) -> ProviderFuture<()> {
        let provider_identifier = self.configuration.provider_identifier;
        let uri = self.configuration.uri;
        let engine = self.engine.clone();

        run_blocking(move || {
            let MachineDefinition {
                laboratory,
                machine,
                image,
            } = definition;
            let connection = Self::connect(provider_identifier, uri)?;

            ensure_laboratory_network_internal(&connection, &laboratory)?;

            if machine_exists_internal(&connection, &machine)? {
                return Err(errors::Provider::MachineAlreadyExists {
                    identifier: machine_name(&machine).to_owned(),
                });
            }

            let domain_xml = render_domain_xml_internal(
                &engine,
                &laboratory,
                &machine,
                &image,
            )?;

            Domain::define_xml(&connection, &domain_xml)
                .map(|_| ())
                .map_err(|error| create_machine_error(&machine, error))
        })
    }

    pub fn start_machine(&self, identifier: Identifier) -> ProviderFuture<()> {
        let provider_identifier = self.configuration.provider_identifier;
        let uri = self.configuration.uri;
        run_blocking(move || {
            let connection = Self::connect(provider_identifier, uri)?;
            let domain = lookup_domain_internal(&connection, &identifier)?;
            if let Some(network_name) = network_name_from_domain_xml(&domain, &identifier)? {
                ensure_network_internal(&connection, &network_name)?;
            }

            if inspect_activity_internal(&domain, &identifier)? {
                return Ok(());
            }

            domain.create().map(|_| ()).map_err(|error| {
                operation_error(
                    "start_machine",
                    &identifier,
                    error,
                    "Verify the domain XML, referenced disk image, and host virtualization configuration before retrying the start operation.",
                )
            })
        })
    }

    pub fn stop_machine(&self, identifier: Identifier, force: bool) -> ProviderFuture<()> {
        let provider_identifier = self.configuration.provider_identifier;
        let uri = self.configuration.uri;
        run_blocking(move || {
            let connection = Self::connect(provider_identifier, uri)?;
            let domain = lookup_domain_internal(&connection, &identifier)?;

            if !inspect_activity_internal(&domain, &identifier)? {
                return Ok(());
            }

            if force {
                domain.destroy().map_err(|error| {
                    operation_error(
                        "stop_machine",
                        &identifier,
                        error,
                        "Retry with libvirt available or inspect the host for stuck QEMU processes.",
                    )
                })
            } else {
                domain.shutdown().map(|_| ()).map_err(|error| {
                    operation_error(
                        "stop_machine",
                        &identifier,
                        error,
                        "Ensure ACPI shutdown is supported by the guest or retry with force enabled.",
                    )
                })
            }
        })
    }

    pub fn destroy_machine(&self, identifier: Identifier) -> ProviderFuture<()> {
        let provider_identifier = self.configuration.provider_identifier;
        let uri = self.configuration.uri;
        run_blocking(move || {
            let connection = Self::connect(provider_identifier, uri)?;
            let domain = lookup_domain_internal(&connection, &identifier)?;

            if inspect_activity_internal(&domain, &identifier)? {
                domain.destroy().map_err(|error| {
                    operation_error(
                        "destroy_machine",
                        &identifier,
                        error,
                        "Stop the guest or inspect libvirt/QEMU state before retrying the destroy operation.",
                    )
                })?;
            }

            domain.undefine().map_err(|error| {
                operation_error(
                    "destroy_machine",
                    &identifier,
                    error,
                    "Remove libvirt references to the domain or inspect snapshots/checkpoints before retrying.",
                )
            })
        })
    }

    pub fn query_machine_state(&self, identifier: Identifier) -> ProviderFuture<state::Machine> {
        let provider_identifier = self.configuration.provider_identifier;
        let uri = self.configuration.uri;
        run_blocking(move || {
            let connection = Self::connect(provider_identifier, uri)?;
            let domain = lookup_domain_internal(&connection, &identifier)?;

            domain
                .get_state()
                .map(|(domain_state, _)| map_domain_state(domain_state))
                .map_err(|error| {
                    operation_error(
                        "query_machine_state",
                        &identifier,
                        error,
                        "Retry after confirming the domain is still reachable through libvirt.",
                    )
                })
        })
    }

    pub fn inject_drive(&self, drive: Drive) -> ProviderFuture<()> {
        let provider_identifier = self.configuration.provider_identifier;
        let uri = self.configuration.uri;
        let engine = self.engine.clone();
        run_blocking(move || {
            let Drive {
                identifier,
                path: drive_path,
            } = drive;
            if !drive_path.exists() {
                return Err(errors::Provider::DriveNotFound {
                    path: drive_path.display().to_string(),
                });
            }

            if !drive_path.is_file() {
                return Err(errors::Provider::DriveInvalid {
                    path: drive_path.display().to_string(),
                });
            }

            let connection = Self::connect(provider_identifier, uri)?;
            let domain = lookup_domain_internal(&connection, &identifier)?;
            let drive_xml = render_drive_xml_internal(&engine, &identifier, &drive_path)?;
            let domain_attachment_flags = domain_attachment_flags_internal(&domain, &identifier)?;

            domain
                    .attach_device_flags(&drive_xml, domain_attachment_flags)
                    .map(|_| ())
                    .map_err(|error| {
                        operation_error(
                            "inject_drive",
                            &identifier,
                            error,
                            "Confirm the drive is a valid readable disk image and retry after ensuring the domain supports SATA/CD-ROM attachment.",
                        )
                    })
        })
    }

    fn connect(
        provider_identifier: &'static str,
        uri: &'static str,
    ) -> Result<Connect, errors::Provider> {
        Connect::open(Some(uri)).map_err(|error| errors::Provider::Unavailable {
            provider: provider_identifier.to_owned(),
            diagnostics: format!(
                "failed to connect to libvirt at '{}': {}",
                uri,
                error.message()
            ),
            remediation: "Ensure libvirtd is running, the selected hypervisor is installed, and the current user has permission to access the libvirt socket.".to_owned(),
        })
    }
}

fn run_blocking<T, F>(operation: F) -> ProviderFuture<T>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, errors::Provider> + Send + 'static,
{
    Box::pin(async move {
        task::spawn_blocking(operation)
            .await
            .map_err(|error| errors::Provider::Internal {
                reason: error.to_string(),
            })?
    })
}

fn machine_name(machine: &Machine) -> &str {
    machine.identifier.value.as_str()
}

fn identifier_name(identifier: &Identifier) -> &str {
    identifier.as_str()
}

fn laboratory_network_name(laboratory: &Laboratory) -> &str {
    if laboratory.is_isolated_network() {
        "default"
    } else {
        laboratory.name.value.as_str()
    }
}

fn lookup_domain_by_name_internal(
    connection: &Connect,
    machine_name: &str,
) -> Result<Domain, errors::Provider> {
    let domain = connection
        .list_all_domains(0)
        .map_err(|error| errors::Provider::OperationFailed {
            operation: "list_machines",
            identifier: machine_name.to_owned(),
            reason: error.message().to_owned(),
            remediation: "Confirm libvirt can list domains on this host before retrying the operation.".to_owned(),
        })?
        .into_iter()
        .find(|domain| domain.get_name().map(|name| name == machine_name).unwrap_or(false));

    domain.ok_or_else(|| errors::Provider::MachineNotFound {
        identifier: machine_name.to_owned(),
    })
}

fn lookup_domain_internal(
    connection: &Connect,
    identifier: &Identifier,
) -> Result<Domain, errors::Provider> {
    lookup_domain_by_name_internal(connection, identifier_name(identifier))
}

fn inspect_activity_internal(
    domain: &Domain,
    identifier: &Identifier,
) -> Result<bool, errors::Provider> {
    domain.is_active().map_err(|error| {
        operation_error(
            "inspect_machine_activity",
            identifier,
            error,
            "Retry after confirming the libvirt domain is still reachable.",
        )
    })
}

fn operation_error(
    operation: &'static str,
    identifier: &Identifier,
    error: error::Error,
    remediation: &'static str,
) -> errors::Provider {
    errors::Provider::OperationFailed {
        operation,
        identifier: identifier_name(identifier).to_owned(),
        reason: error.message().to_owned(),
        remediation: remediation.to_owned(),
    }
}

fn map_domain_state(domain_state: sys::virDomainState) -> state::Machine {
    match domain_state {
        sys::VIR_DOMAIN_RUNNING => state::Machine::Provisioning,
        sys::VIR_DOMAIN_BLOCKED | sys::VIR_DOMAIN_PAUSED | sys::VIR_DOMAIN_PMSUSPENDED => {
            state::Machine::Provisioning
        }
        sys::VIR_DOMAIN_SHUTDOWN | sys::VIR_DOMAIN_SHUTOFF => state::Machine::Planned,
        sys::VIR_DOMAIN_CRASHED => state::Machine::Failed,
        sys::VIR_DOMAIN_NOSTATE | _ => state::Machine::Provisioning,
    }
}

fn render_domain_xml_internal(
    engine: &Engine,
    laboratory: &Laboratory,
    machine: &Machine,
    image: &Path,
) -> Result<String, errors::Provider> {
    let image = image.display().to_string();
    let is_windows = matches!(machine.operating_system, OperatingSystem::Windows { .. });

    let mut context = Context::new();
    context.insert("identifier", machine_name(machine));
    context.insert("name", machine.name.as_str());
    context.insert("memory", &machine.hardware.memory_megabyte.value);
    context.insert("cpus", &machine.hardware.cpus.value);
    context.insert("image", &image);
    context.insert("network_name", laboratory_network_name(laboratory));
    context.insert("is_windows", &is_windows);

    engine
        .render("libvirt/kvm/machine.xml", &context)
        .map_err(|source| errors::Provider::Template { source })
}

fn render_drive_xml_internal(
    engine: &Engine,
    identifier: &Identifier,
    drive_path: &Path,
) -> Result<String, errors::Provider> {
    let drive_path = drive_path.display().to_string();

    let mut context = Context::new();
    context.insert("path", &drive_path);

    engine
        .render("libvirt/kvm/configuration-drive.xml", &context)
        .map_err(|source| errors::Provider::Template { source })
}

fn ensure_laboratory_network_internal(
    connection: &Connect,
    laboratory: &Laboratory,
) -> Result<(), errors::Provider> {
    if laboratory.is_isolated_network() {
        return Ok(());
    }

    ensure_network_internal(connection, laboratory_network_name(laboratory))
}

fn ensure_network_internal(
    connection: &Connect,
    network_name: &str,
) -> Result<(), errors::Provider> {
    let network = connection
        .list_all_networks(0)
        .map_err(|error| errors::Provider::OperationFailed {
            operation: "list_networks",
            identifier: network_name.to_owned(),
            reason: error.message().to_owned(),
            remediation: "Confirm libvirt can list networks on this host before retrying the operation.".to_owned(),
        })?
        .into_iter()
        .find(|network| network.get_name().map(|name| name == network_name).unwrap_or(false));

    match network {
        Some(network) => {
            if !network
                .is_active()
                .map_err(|error| network_operation_error(network_name, "inspect_network", error))?
            {
                network.create().map_err(|error| {
                    network_operation_error(network_name, "start_network", error)
                })?;
            }

            if network.is_persistent().unwrap_or(false) {
                let _ = network.set_autostart(true);
            }

            Ok(())
        }
        None => {
            let network_xml = render_network_xml_internal(network_name);
            let network = LibvirtNetwork::define_xml(connection, &network_xml)
                .map_err(|error| network_operation_error(network_name, "create_network", error))?;

            network
                .create()
                .map_err(|error| network_operation_error(network_name, "start_network", error))?;

            let _ = network.set_autostart(true);

            Ok(())
        }
    }
}

fn render_network_xml_internal(network_name: &str) -> String {
    let bridge_name = bridge_name_internal(network_name);
    let subnet = network_subnet_internal(network_name);

    format!(
        "<network><name>{network_name}</name><forward mode='nat'/><bridge name='{bridge_name}' stp='on' delay='0'/><ip address='{subnet}.1' netmask='255.255.255.0'><dhcp><range start='{subnet}.2' end='{subnet}.254'/></dhcp></ip></network>"
    )
}

fn bridge_name_internal(network_name: &str) -> String {
    let mut name = network_name
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .take(12)
        .collect::<String>();

    if name.is_empty() {
        name.push_str("malina");
    }

    format!("br-{name}")
}

fn network_subnet_internal(network_name: &str) -> String {
    let mut hash: u32 = 0x811c9dc5;

    for byte in network_name.as_bytes() {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }

    let third_octet = ((hash >> 8) as u8).max(16);

    format!("10.{}.{}", 16 + (third_octet % 240), hash as u8)
}

fn network_operation_error(
    network_name: &str,
    operation: &'static str,
    error: error::Error,
) -> errors::Provider {
    errors::Provider::OperationFailed {
        operation,
        identifier: network_name.to_owned(),
        reason: error.message().to_owned(),
        remediation: "Check libvirt network permissions and retry after ensuring the host can create or start the laboratory network.".to_owned(),
    }
}

fn network_name_from_domain_xml(
    domain: &Domain,
    identifier: &Identifier,
) -> Result<Option<String>, errors::Provider> {
    let xml = domain
        .get_xml_desc(0)
        .map_err(|error| {
            operation_error(
                "inspect_domain_xml",
                identifier,
                error,
                "Retry after confirming the domain XML is accessible.",
            )
        })?;

    let needle = "network='";
    let Some(start) = xml.find(needle) else {
        return Ok(None);
    };

    let remainder = &xml[start + needle.len()..];
    let Some(end) = remainder.find('\'') else {
        return Ok(None);
    };

    Ok(Some(remainder[..end].to_owned()))
}

fn domain_attachment_flags_internal(
    domain: &Domain,
    identifier: &Identifier,
) -> Result<u32, errors::Provider> {
    inspect_activity_internal(domain, identifier).map(|is_active| {
        if is_active {
            (sys::VIR_DOMAIN_AFFECT_CONFIG | sys::VIR_DOMAIN_AFFECT_LIVE) as u32
        } else {
            sys::VIR_DOMAIN_AFFECT_CONFIG as u32
        }
    })
}

fn machine_exists_internal(
    connection: &Connect,
    machine: &Machine,
) -> Result<bool, errors::Provider> {
    Ok(connection
        .list_all_domains(0)
        .map_err(|error| errors::Provider::OperationFailed {
            operation: "list_machines",
            identifier: machine_name(machine).to_owned(),
            reason: error.message().to_owned(),
            remediation: "Confirm libvirt can list domains on this host before retrying the operation.".to_owned(),
        })?
        .into_iter()
        .any(|domain| {
            domain
                .get_name()
                .map(|name| name == machine_name(machine))
                .unwrap_or(false)
        }))
}

fn create_machine_error(machine: &Machine, error: error::Error) -> errors::Provider {
    match error.code() {
        error::ErrorNumber::NoMemory => errors::Provider::ResourceExhausted {
            resource_type: "memory",
            available: 0,
            required: u64::from(machine.hardware.memory_megabyte.value),
        },
        _ => errors::Provider::OperationFailed {
            operation: "create_machine",
            identifier: machine_name(machine).to_owned(),
            reason: error.message().to_owned(),
            remediation: "Check the generated libvirt domain settings, host virtualization support, and referenced disk image before retrying.".to_owned(),
        },
    }
}
