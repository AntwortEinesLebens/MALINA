// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use miette::Diagnostic;
use std::io::Error as IoError;
use thiserror::Error;
use toml::de::Error as DeserializeError;

#[derive(Error, Diagnostic, Debug)]
pub enum Validation {
    #[diagnostic(help("Ensure the configuration file exists at the specified path."))]
    #[error("Configuration file not found: {path}")]
    ConfigurationNotFound { path: String },

    #[diagnostic(help(
        "Fix the TOML syntax error. Check for: missing quotes, invalid brackets, typos in field names"
    ))]
    #[error("TOML syntax error in configuration")]
    InvalidTomlSyntax {
        #[source]
        source: DeserializeError,
    },

    #[diagnostic(help("Ensure the file exists and you have read permissions."))]
    #[error("Failed to read configuration file: {path}")]
    ConfigurationReadError {
        path: String,
        #[source]
        source: IoError,
    },

    #[diagnostic(help("The specified path must be a file, not a directory."))]
    #[error("Configuration path is not a file: {path}")]
    ConfigurationPathIsDirectory { path: String },

    #[diagnostic(help(
        "Configuration files must be valid UTF-8 text. Ensure the file encoding is UTF-8."
    ))]
    #[error("Configuration file contains invalid UTF-8 content: {path}")]
    ConfigurationInvalidUtf8 { path: String },
}
