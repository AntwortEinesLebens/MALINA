// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{errors::Validation, laboratories::machines::hardware::HostResources};
pub use laboratory::Laboratory;
pub use machines::machine::Machine;
use std::path::Path;
use toml_span::{DeserError, Deserialize, de_helpers::TableHelper};

mod laboratory;
pub mod machines;

#[derive(Debug)]
pub struct Configuration {
    pub version: String,
    pub laboratory: Laboratory,
    pub machines: Vec<Machine>,
}

impl<'de> Deserialize<'de> for Configuration {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let version = table.required("version")?;
        let laboratory = table.required("laboratory")?;
        let machines = table.required("machines")?;
        table.finalize(None)?;

        Ok(Self {
            version,
            laboratory,
            machines,
        })
    }
}

impl Configuration {
    pub fn parse(source_name: &str, source_code: &str) -> Result<Self, Validation> {
        let mut value = toml_span::parse(source_code)
            .map_err(|error| Validation::from_toml_error(error, source_name, source_code))?;

        Self::deserialize(&mut value).map_err(|error| {
            Validation::from_toml_deserialize_error(error, source_name, source_code)
        })
    }

    pub fn validate(
        &self,
        source_name: &str,
        source_code: &str,
        parent: &Path,
    ) -> Result<(), Validation> {
        let host_resources = HostResources::detect();

        for machine in &self.machines {
            machine.validate(source_name, source_code, parent, &host_resources)?;
        }

        Ok(())
    }
}
