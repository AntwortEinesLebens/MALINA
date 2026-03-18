// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::errors::{Validation, validation};
use miette::NamedSource;
use sysinfo::System;
use toml_span::{DeserError, Deserialize, Spanned, de_helpers::TableHelper};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostResources {
    cpus: usize,
    memory_megabyte: u64,
}

impl HostResources {
    pub fn detect() -> Self {
        let mut system = System::new();
        system.refresh_cpu_all();
        system.refresh_memory();

        Self {
            cpus: system.cpus().len(),
            memory_megabyte: system.total_memory() / (1024 * 1024),
        }
    }
}

#[derive(Debug)]
pub struct Hardware {
    pub cpus: Spanned<u32>,
    pub memory_megabyte: Spanned<u32>,
}

impl<'de> Deserialize<'de> for Hardware {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let cpus = table.required_s("cpus")?;
        let memory_megabyte = table.required_s("memory_megabyte")?;
        table.finalize(None)?;

        Ok(Self {
            cpus,
            memory_megabyte,
        })
    }
}

impl Hardware {
    const MINIMUM_CPUS: u32 = 1;
    const MINIMUM_MEMORY_MEGABYTE: u32 = 512;

    pub fn validate(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
        host_resources: &HostResources,
    ) -> Result<(), Validation> {
        self.validate_cpu(machine_identifier, source_name, source_code, host_resources)?;
        self.validate_memory(machine_identifier, source_name, source_code, host_resources)?;

        Ok(())
    }

    fn validate_cpu(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
        host_resources: &HostResources,
    ) -> Result<(), Validation> {
        let cpus = self.cpus.value;

        if cpus < Self::MINIMUM_CPUS {
            return Err(Validation::InsufficientCpu {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.cpus.span),
                machine_identifier: machine_identifier.to_owned(),
                actual: cpus,
            });
        }

        if cpus as usize > host_resources.cpus {
            return Err(Validation::ExcessiveCpu {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.cpus.span),
                machine_identifier: machine_identifier.to_owned(),
                requested: cpus,
                host_cpus: host_resources.cpus,
            });
        }

        Ok(())
    }

    fn validate_memory(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
        host_resources: &HostResources,
    ) -> Result<(), Validation> {
        let memory_megabyte = self.memory_megabyte.value;

        if memory_megabyte < Self::MINIMUM_MEMORY_MEGABYTE {
            return Err(Validation::InsufficientMemory {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.memory_megabyte.span),
                machine_identifier: machine_identifier.to_owned(),
                actual: memory_megabyte,
                minimum: Self::MINIMUM_MEMORY_MEGABYTE,
            });
        }

        if u64::from(memory_megabyte) > host_resources.memory_megabyte {
            return Err(Validation::ExcessiveMemory {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.memory_megabyte.span),
                machine_identifier: machine_identifier.to_owned(),
                requested: memory_megabyte,
                host_memory_megabyte: host_resources.memory_megabyte,
            });
        }

        Ok(())
    }
}
