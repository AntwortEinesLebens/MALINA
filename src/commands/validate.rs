// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{errors::Validation, laboratories::Configuration, logger::Logger};
use miette::{NamedSource, Result};
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

    let source_code = fs::read_to_string(&path).map_err(|error| {
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

    let source_name = path.display().to_string();

    let configuration: Configuration =
        toml::from_str(&source_code).map_err(|error| Validation::InvalidTomlSyntax {
            message: error.message().to_string(),
            source_code: NamedSource::new(source_name.clone(), source_code.clone()),
            span: error.span().map(Into::into),
        })?;

    configuration.validate(&source_name, &source_code)?;

    Logger::print(&format!(
        "Configuration is valid - Laboratory: {}",
        configuration.laboratory.name
    ));

    Ok(())
}
