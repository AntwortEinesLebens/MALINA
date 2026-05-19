// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::errors::Template;
use base64::engine::general_purpose;
use std::collections::HashMap;
use tera::{Context, Filter, Tera, Value};

#[derive(Debug, Clone)]
pub struct Engine {
    tera: Tera,
}

impl Engine {
    const CLOUD_INIT_META_DATA_TEMPLATE: &'static str =
        include_str!("../templates/cloud-init/meta-data");
    const CLOUD_INIT_USER_DATA_TEMPLATE: &'static str =
        include_str!("../templates/cloud-init/user-data");
    const CLOUDBASE_INIT_META_DATA_TEMPLATE: &'static str =
        include_str!("../templates/cloudbase-init/meta-data");
    const CLOUDBASE_INIT_CONFIGURATION_TEMPLATE: &'static str =
        include_str!("../templates/cloudbase-init/cloudbase-init.conf");
    const CLOUDBASE_INIT_USER_DATA_TEMPLATE: &'static str =
        include_str!("../templates/cloudbase-init/user-data");
    const LIBVIRT_KVM_MACHINE_TEMPLATE: &'static str =
        include_str!("../templates/libvirt/kvm/machine.xml");
    const LIBVIRT_KVM_CONFIGURATION_DRIVE_TEMPLATE: &'static str =
        include_str!("../templates/libvirt/kvm/configuration-drive.xml");

    pub fn new() -> Result<Self, Template> {
        let mut tera = Tera::default();
        tera.register_filter("powershell_base64_encode", PowershellBase64Encoder);
        tera.register_filter("yaml_escape", YamlScalarEncoder);

        for (name, template) in [
            ("cloud-init/meta-data", Self::CLOUD_INIT_META_DATA_TEMPLATE),
            ("cloud-init/user-data", Self::CLOUD_INIT_USER_DATA_TEMPLATE),
            (
                "cloudbase-init/meta-data",
                Self::CLOUDBASE_INIT_META_DATA_TEMPLATE,
            ),
            (
                "cloudbase-init/cloudbase-init.conf",
                Self::CLOUDBASE_INIT_CONFIGURATION_TEMPLATE,
            ),
            (
                "cloudbase-init/user-data",
                Self::CLOUDBASE_INIT_USER_DATA_TEMPLATE,
            ),
            (
                "libvirt/kvm/machine.xml",
                Self::LIBVIRT_KVM_MACHINE_TEMPLATE,
            ),
            (
                "libvirt/kvm/configuration-drive.xml",
                Self::LIBVIRT_KVM_CONFIGURATION_DRIVE_TEMPLATE,
            ),
        ] {
            tera.add_raw_template(name, template)
                .map_err(|error| Template::Registration {
                    name,
                    reason: error.to_string(),
                })?;
        }

        Ok(Self { tera })
    }

    pub fn render(&self, name: &'static str, context: &Context) -> Result<String, Template> {
        self.tera
            .render(name, context)
            .map_err(|error| Template::Render {
                name,
                reason: error.to_string(),
            })
    }
}

#[derive(Debug, Clone, Copy)]
struct PowershellBase64Encoder;

impl Filter for PowershellBase64Encoder {
    fn filter(&self, value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
        let script = value
            .as_str()
            .ok_or_else(|| tera::Error::msg("expected string"))?;
        let utf16 = script
            .encode_utf16()
            .flat_map(|unit| unit.to_le_bytes())
            .collect::<Vec<_>>();
        let encoded = base64::Engine::encode(&general_purpose::STANDARD, utf16);

        tera::to_value(encoded).map_err(|error| tera::Error::msg(error.to_string()))
    }
}

#[derive(Debug, Clone, Copy)]
struct YamlScalarEncoder;

impl Filter for YamlScalarEncoder {
    fn filter(&self, value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
        let scalar = value
            .as_str()
            .ok_or_else(|| tera::Error::msg("expected string"))?;
        let encoded = serde_yaml::to_string(scalar)
            .map_err(|error| tera::Error::msg(error.to_string()))?
            .to_owned();

        tera::to_value(encoded).map_err(|error| tera::Error::msg(error.to_string()))
    }
}
