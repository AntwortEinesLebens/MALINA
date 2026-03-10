// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Validation {
    #[diagnostic(help("Ensure the configuration file exists at the specified path."))]
    #[error("Configuration file not found: {path}")]
    ConfigurationNotFound { path: String },
}
