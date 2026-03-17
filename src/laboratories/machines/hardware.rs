// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::errors::Validation;
use miette::NamedSource;
use serde::Deserialize;
use sysinfo::System;
use toml::Spanned;

const MINIMUM_CPUS: u32 = 1;
const MINIMUM_MEMORY_MEGABYTE: u32 = 512;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hardware {
    pub cpus: Spanned<u32>,
    pub memory_megabyte: Spanned<u32>,
}

impl Hardware {
    pub fn validate(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
    ) -> Result<(), Validation> {
        let mut system = System::new();
        system.refresh_cpu_all();
        system.refresh_memory();

        self.validate_cpu(machine_identifier, source_name, source_code, &system)?;
        self.validate_memory(machine_identifier, source_name, source_code, &system)?;

        Ok(())
    }

    fn validate_cpu(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
        system: &System,
    ) -> Result<(), Validation> {
        let cpus = *self.cpus.get_ref();

        if cpus < MINIMUM_CPUS {
            return Err(Validation::InsufficientCpu {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: self.cpus.span().into(),
                machine_identifier: machine_identifier.to_owned(),
                actual: cpus,
            });
        }

        let host_cpus = system.cpus().len();

        if cpus as usize > host_cpus {
            return Err(Validation::ExcessiveCpu {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: self.cpus.span().into(),
                machine_identifier: machine_identifier.to_owned(),
                requested: cpus,
                host_cpus,
            });
        }

        Ok(())
    }

    fn validate_memory(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
        system: &System,
    ) -> Result<(), Validation> {
        let memory_megabyte = *self.memory_megabyte.get_ref();

        if memory_megabyte < MINIMUM_MEMORY_MEGABYTE {
            return Err(Validation::InsufficientMemory {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: self.memory_megabyte.span().into(),
                machine_identifier: machine_identifier.to_owned(),
                actual: memory_megabyte,
                minimum: MINIMUM_MEMORY_MEGABYTE,
            });
        }

        let host_memory_megabyte = system.total_memory() / (1024 * 1024);

        if u64::from(memory_megabyte) > host_memory_megabyte {
            return Err(Validation::ExcessiveMemory {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: self.memory_megabyte.span().into(),
                machine_identifier: machine_identifier.to_owned(),
                requested: memory_megabyte,
                host_memory_megabyte,
            });
        }

        Ok(())
    }
}
