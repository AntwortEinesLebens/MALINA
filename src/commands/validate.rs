// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{errors::Validation, laboratories::Configuration, logger::Logger};
use miette::Result;
use std::{fs, io::ErrorKind as IoErrorKind, path::PathBuf};

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

    let source_code = fs::read_to_string(&path).map_err(|error| {
        let kind = error.kind();

        if kind == IoErrorKind::InvalidData {
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

    let source_name = path.display().to_string();
    let parent = path.parent().unwrap_or(path.as_path());

    let configuration = Configuration::parse(&source_name, &source_code)?;
    configuration.validate(&source_name, &source_code, parent)?;

    Logger::print(&format!(
        "Configuration is valid - Laboratory: {}",
        configuration.laboratory.name
    ));

    Ok(())
}
