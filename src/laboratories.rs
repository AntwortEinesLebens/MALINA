// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::errors::Validation;
pub use laboratory::Laboratory;
pub use machines::machine::Machine;
use serde::Deserialize;

mod laboratory;
pub mod machines;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub version: String,
    pub laboratory: Laboratory,
    pub machines: Vec<Machine>,
}

impl Configuration {
    pub fn validate(&self, source_name: &str, source_code: &str) -> Result<(), Validation> {
        for machine in &self.machines {
            machine.validate(source_name, source_code)?;
        }

        Ok(())
    }
}
