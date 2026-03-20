// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    errors::{Validation, validation},
    laboratories::machines::hardware::HostResources,
    logger::Logger,
};
pub use laboratory::Laboratory;
pub use machines::machine::Machine;
use miette::NamedSource;
use std::{collections::HashMap, path::Path};
use toml_span::{DeserError, Deserialize, Spanned, de_helpers::TableHelper};

mod laboratory;
pub mod machines;

#[derive(Debug)]
pub struct Configuration {
    pub version: String,
    pub laboratory: Laboratory,
    pub machines: Spanned<Vec<Machine>>,
}

impl<'de> Deserialize<'de> for Configuration {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let version = table.required("version")?;
        let laboratory = table.required("laboratory")?;
        let machines = table.required_s::<Vec<Machine>>("machines")?;
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
        if self.machines.value.is_empty() {
            return Err(Validation::EmptyMachines {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.machines.span),
            });
        }

        let mut identifier_first_indices: HashMap<&str, usize> =
            HashMap::with_capacity(self.machines.value.len());
        let host_resources = HostResources::detect();

        for (index, machine) in self.machines.value.iter().enumerate() {
            Logger::info(&format!(
                "Machine {} [{}/{}]",
                machine.identifier.value.trim(),
                index + 1,
                self.machines.value.len()
            ));

            machine.validate(source_name, source_code, parent, &host_resources)?;

            let identifier = machine.identifier.value.trim();

            if let Some(&first_index) = identifier_first_indices.get(identifier) {
                return Err(Validation::DuplicateMachineIdentifier {
                    source_code: NamedSource::new(source_name, source_code.to_owned()),
                    span: validation::to_source_span(machine.identifier.span),
                    identifier: identifier.to_owned(),
                    indices: format!("{}, {}", first_index, index),
                });
            }
            identifier_first_indices.insert(identifier, index);
        }

        Ok(())
    }
}
