// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{errors::Validation, laboratories::machines::hardware::Hardware};
use serde::Deserialize;
use toml::Spanned;

#[derive(Debug, Deserialize)]
pub struct Machine {
    pub identifier: Spanned<String>,
    pub name: String,
    pub hardware: Hardware,
}

impl Machine {
    pub fn validate(&self, source_name: &str, source_code: &str) -> Result<(), Validation> {
        let identifier = self.identifier.get_ref();
        self.hardware
            .validate(identifier, source_name, source_code)?;

        Ok(())
    }
}
