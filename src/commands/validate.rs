// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{errors::Validation, logger::Logger};
use miette::Result;
use std::path::PathBuf;

pub fn execute(path: PathBuf) -> Result<()> {
    Logger::info(&format!("Validating: {}", path.display()));

    if !path.exists() {
        return Err(Validation::ConfigurationNotFound {
            path: path.display().to_string(),
        }
        .into());
    }

    Logger::print("Configuration is valid");

    Ok(())
}
