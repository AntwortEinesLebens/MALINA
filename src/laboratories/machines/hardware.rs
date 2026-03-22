// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    errors::{Validation, validation},
    logger::Logger,
};
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

#[derive(Debug, Clone)]
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
    const RECOMMENDED_MINIMUM_CPUS: u32 = 4;
    const RECOMMENDED_MINIMUM_MEMORY_MEGABYTE: u32 = 8192;

    pub fn validate(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
        host_resources: &HostResources,
    ) -> Result<(), Validation> {
        self.validate_cpu(machine_identifier, source_name, source_code, host_resources)?;
        self.validate_memory(machine_identifier, source_name, source_code, host_resources)?;
        self.check_warnings(machine_identifier);

        Ok(())
    }

    fn check_warnings(&self, machine_identifier: &str) {
        if self.memory_megabyte.value < Self::RECOMMENDED_MINIMUM_MEMORY_MEGABYTE {
            Logger::warn(&format!(
                "Machine '{}' memory {}MB ({}MB recommended) may trigger VM detection",
                machine_identifier,
                self.memory_megabyte.value,
                Self::RECOMMENDED_MINIMUM_MEMORY_MEGABYTE,
            ));
        }

        if self.cpus.value < Self::RECOMMENDED_MINIMUM_CPUS {
            Logger::warn(&format!(
                "Machine '{}' has {} CPUs ({} recommended) may trigger VM detection",
                machine_identifier,
                self.cpus.value,
                Self::RECOMMENDED_MINIMUM_CPUS,
            ));
        }
    }

    fn validate_cpu(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
        host_resources: &HostResources,
    ) -> Result<(), Validation> {
        if self.cpus.value < Self::MINIMUM_CPUS {
            return Err(Validation::InsufficientCpu {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.cpus.span),
                machine_identifier: machine_identifier.to_owned(),
                actual: self.cpus.value,
            });
        }

        if self.cpus.value as usize > host_resources.cpus {
            return Err(Validation::ExcessiveCpu {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.cpus.span),
                machine_identifier: machine_identifier.to_owned(),
                requested: self.cpus.value,
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
        if self.memory_megabyte.value < Self::MINIMUM_MEMORY_MEGABYTE {
            return Err(Validation::InsufficientMemory {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.memory_megabyte.span),
                machine_identifier: machine_identifier.to_owned(),
                actual: self.memory_megabyte.value,
                minimum: Self::MINIMUM_MEMORY_MEGABYTE,
            });
        }

        if u64::from(self.memory_megabyte.value) > host_resources.memory_megabyte {
            return Err(Validation::ExcessiveMemory {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.memory_megabyte.span),
                machine_identifier: machine_identifier.to_owned(),
                requested: self.memory_megabyte.value,
                host_memory_megabyte: host_resources.memory_megabyte,
            });
        }

        Ok(())
    }
}
