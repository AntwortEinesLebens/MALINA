// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{errors::Validation, laboratories::Configuration, logger::Logger};
use miette::Result;
use std::{fs, io::ErrorKind, path::PathBuf};

pub fn execute(path: PathBuf) -> Result<()> {
    Logger::info(&format!("Validating: {}", path.display()));

    if !path.exists() {
        return Err(Validation::ConfigurationNotFound {
            path: path.display().to_string(),
        }
        .into());
    }

    if path.is_dir() {
        return Err(Validation::ConfigurationPathIsDirectory {
            path: path.display().to_string(),
        }
        .into());
    }

    let content = fs::read_to_string(&path).map_err(|error| {
        let error_kind = error.kind();

        if error_kind == ErrorKind::InvalidData {
            Validation::ConfigurationInvalidUtf8 {
                path: path.display().to_string(),
            }
        } else {
            Validation::ConfigurationReadError {
                path: path.display().to_string(),
                source: error,
            }
        }
    })?;

    let config: Configuration = toml::from_str(&content)
        .map_err(|error| Validation::InvalidTomlSyntax { source: error })?;

    Logger::print(&format!(
        "Configuration is valid - Laboratory: {}",
        config.laboratory.name
    ));

    Ok(())
}
