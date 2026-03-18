// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    errors::Validation,
    laboratories::machines::{
        hardware::{Hardware, HostResources},
        operating_system::OperatingSystem,
    },
};
use std::path::Path;
use toml_span::{DeserError, Deserialize, Spanned, de_helpers::TableHelper};

#[derive(Debug)]
pub struct Machine {
    pub identifier: Spanned<String>,
    pub name: String,
    pub hardware: Hardware,
    pub operating_system: OperatingSystem,
}

impl<'de> Deserialize<'de> for Machine {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let identifier = table.required_s("identifier")?;
        let name = table.required("name")?;
        let hardware = table.required("hardware")?;
        let operating_system = table.required("operating_system")?;
        table.finalize(None)?;

        Ok(Self {
            identifier,
            name,
            hardware,
            operating_system,
        })
    }
}

impl Machine {
    pub fn validate(
        &self,
        source_name: &str,
        source_code: &str,
        parent: &Path,
        host_resources: &HostResources,
    ) -> Result<(), Validation> {
        let identifier = self.identifier.value.as_str();
        self.hardware
            .validate(identifier, source_name, source_code, host_resources)?;
        self.operating_system
            .validate(identifier, source_name, source_code, parent)?;

        Ok(())
    }
}
