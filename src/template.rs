// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{errors::Template, laboratories::Provider};
use tera::{Context, Tera};

const KVM_MACHINE_TEMPLATE: &str = include_str!("../templates/libvirt/kvm/machine.xml");
const KVM_CONFIGURATION_DRIVE_TEMPLATE: &str =
    include_str!("../templates/libvirt/kvm/configuration-drive.xml");

#[derive(Debug, Clone)]
pub struct Engine {
    tera: Tera,
}

impl Engine {
    pub fn new(provider: Provider) -> Result<Self, Template> {
        let mut tera = Tera::default();

        match provider {
            Provider::Kvm => {
                tera.add_raw_template("machine.xml", KVM_MACHINE_TEMPLATE)
                    .map_err(|error| Template::Registration {
                        name: "machine.xml",
                        reason: error.to_string(),
                    })?;
                tera.add_raw_template("configuration-drive.xml", KVM_CONFIGURATION_DRIVE_TEMPLATE)
                    .map_err(|error| Template::Registration {
                        name: "configuration-drive.xml",
                        reason: error.to_string(),
                    })?;
            }
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
