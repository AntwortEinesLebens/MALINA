// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use miette::{Diagnostic, NamedSource, SourceSpan};
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Validation {
    #[diagnostic(help("Ensure the configuration file exists at the specified path."))]
    #[error("Configuration file not found: {path}")]
    ConfigurationNotFound { path: String },

    #[diagnostic(help(
        "Fix the TOML syntax error. Check for: missing quotes, invalid brackets, typos in field names"
    ))]
    #[error("TOML syntax error in configuration: {message}")]
    InvalidTomlSyntax {
        message: String,
        #[source_code]
        source_code: NamedSource<String>,
        #[label("syntax error")]
        span: Option<SourceSpan>,
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

    #[diagnostic(help(
        "Allocate at least 1 CPU to the machine. Virtual machines require at least one CPU to function."
    ))]
    #[error("Machine '{machine_identifier}' has insufficient CPUs: {actual} (minimum: 1)")]
    InsufficientCpu {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`cpus` must be at least 1")]
        span: SourceSpan,
        machine_identifier: String,
        actual: u32,
    },

    #[diagnostic(help(
        "Allocate at least {minimum} MB of memory. This machine requires more RAM to operate reliably."
    ))]
    #[error(
        "Machine '{machine_identifier}' has insufficient memory: {actual} MB (minimum: {minimum} MB)"
    )]
    InsufficientMemory {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`memory_megabyte` must be at least {minimum}")]
        span: SourceSpan,
        machine_identifier: String,
        actual: u32,
        minimum: u32,
    },

    #[diagnostic(help(
        "Reduce CPU count to {host_cpus} or fewer. This host has {host_cpus} logical CPUs available."
    ))]
    #[error(
        "Machine '{machine_identifier}' requests more CPUs than available: {requested} (host has {host_cpus})"
    )]
    ExcessiveCpu {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`cpus` exceeds the host limit")]
        span: SourceSpan,
        machine_identifier: String,
        requested: u32,
        host_cpus: usize,
    },
}
