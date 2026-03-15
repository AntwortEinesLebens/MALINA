// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{errors::validation::Validation, laboratories::Machine};
use miette::NamedSource;

const MINIMUM_CPUS: u32 = 1;
const MINIMUM_MEMORY_MEGABYTE: u32 = 512;

pub fn validate_hardware(
    machine: &Machine,
    host_cpus: usize,
    source_name: &str,
    source_code: &str,
) -> Result<(), Validation> {
    let hardware = &machine.hardware;
    let identifier = machine.identifier.get_ref().clone();
    let cpus = *hardware.cpus.get_ref();

    if cpus < MINIMUM_CPUS {
        return Err(Validation::InsufficientCpu {
            source_code: NamedSource::new(source_name, source_code.to_owned()),
            span: hardware.cpus.span().into(),
            machine_identifier: identifier.clone(),
            actual: cpus,
        });
    }

    let memory_megabyte = *hardware.memory_megabyte.get_ref();

    if memory_megabyte < MINIMUM_MEMORY_MEGABYTE {
        return Err(Validation::InsufficientMemory {
            source_code: NamedSource::new(source_name, source_code.to_owned()),
            span: hardware.memory_megabyte.span().into(),
            machine_identifier: identifier.clone(),
            actual: memory_megabyte,
            minimum: MINIMUM_MEMORY_MEGABYTE,
        });
    }

    if cpus as usize > host_cpus {
        return Err(Validation::ExcessiveCpu {
            source_code: NamedSource::new(source_name, source_code.to_owned()),
            span: hardware.cpus.span().into(),
            machine_identifier: identifier,
            requested: cpus,
            host_cpus,
        });
    }

    Ok(())
}

pub fn validate_all_machines(
    machines: &[Machine],
    source_name: &str,
    source_code: &str,
) -> Result<(), Validation> {
    let host_cpus = num_cpus::get();

    for machine in machines {
        validate_hardware(machine, host_cpus, source_name, source_code)?;
    }

    Ok(())
}
