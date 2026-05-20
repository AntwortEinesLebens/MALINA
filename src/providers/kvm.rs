// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    errors::Template,
    laboratories::Identifier,
    providers::{Drive, MachineDefinition, Provider, ProviderFuture, libvirt},
    state,
};

const LIBVIRT_CONFIGURATION: libvirt::Configuration = libvirt::Configuration {
    provider_identifier: "kvm",
    uri: "qemu:///system",
};

#[derive(Debug, Clone)]
pub struct Kvm {
    libvirt: libvirt::Libvirt,
}

impl Kvm {
    pub fn new() -> Result<Self, Template> {
        Ok(Self {
            libvirt: libvirt::Libvirt::new(LIBVIRT_CONFIGURATION)?,
        })
    }
}

impl Provider for Kvm {
    fn identifier(&self) -> &'static str {
        "kvm"
    }

    fn verify_availability(&self) -> ProviderFuture<()> {
        self.libvirt.verify_availability()
    }

    fn create_machine(&self, definition: MachineDefinition) -> ProviderFuture<()> {
        self.libvirt.create_machine(definition)
    }

    fn start_machine(&self, identifier: Identifier) -> ProviderFuture<()> {
        self.libvirt.start_machine(identifier)
    }

    fn stop_machine(&self, identifier: Identifier, force: bool) -> ProviderFuture<()> {
        self.libvirt.stop_machine(identifier, force)
    }

    fn destroy_machine(&self, identifier: Identifier) -> ProviderFuture<()> {
        self.libvirt.destroy_machine(identifier)
    }

    fn query_machine_state(&self, identifier: Identifier) -> ProviderFuture<state::Machine> {
        self.libvirt.query_machine_state(identifier)
    }

    fn inject_drive(&self, drive: Drive) -> ProviderFuture<()> {
        self.libvirt.inject_drive(drive)
    }
}
