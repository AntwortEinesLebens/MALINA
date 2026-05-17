// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use miette::{Diagnostic, NamedSource, SourceSpan};
use std::io;
use thiserror::Error;
use toml_span::{DeserError, Span};

#[derive(Error, Diagnostic, Debug)]
pub enum Validation {
    #[diagnostic(help("Ensure the configuration file exists at the specified path."))]
    #[error("Configuration file not found: {path}")]
    ConfigurationNotFound { path: String },

    #[diagnostic(help("Fix the configuration file syntax or unsupported field values."))]
    #[error("Invalid configuration: {message}")]
    InvalidConfiguration {
        message: String,
        #[source_code]
        source_code: NamedSource<String>,
        #[label("invalid configuration")]
        span: Option<SourceSpan>,
    },

    #[diagnostic(help("Ensure the file exists and you have read permissions."))]
    #[error("Failed to read configuration file: {path}")]
    ConfigurationReadError {
        path: String,
        #[source]
        source: io::Error,
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
        "Reduce CPU count to {host_cpus} or fewer. This host has {host_cpus} total logical CPUs."
    ))]
    #[error(
        "Machine '{machine_identifier}' requests more CPUs than the host total: {requested} (host has {host_cpus})"
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

    #[diagnostic(help(
        "Reduce memory to {host_memory_megabyte} MB or less. This host has {host_memory_megabyte} MB of total memory."
    ))]
    #[error(
        "Machine '{machine_identifier}' requests more memory than the host total: {requested} MB (host has {host_memory_megabyte} MB)"
    )]
    ExcessiveMemory {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`memory_megabyte` exceeds the host limit")]
        span: SourceSpan,
        machine_identifier: String,
        requested: u32,
        host_memory_megabyte: u64,
    },

    #[diagnostic(help("Use a supported image extension. Valid extensions: qcow2"))]
    #[error("Machine '{machine_identifier}' has invalid image extension: '{actual}'")]
    InvalidImageExtension {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`image` must use extension: qcow2")]
        span: SourceSpan,
        machine_identifier: String,
        actual: String,
    },

    #[diagnostic(help("Provide a non-empty path for the machine image."))]
    #[error("Machine '{machine_identifier}' has an empty image path")]
    EmptyImagePath {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`image` must not be empty")]
        span: SourceSpan,
        machine_identifier: String,
    },

    #[diagnostic(help("Ensure the image file exists at the specified path."))]
    #[error("Machine '{machine_identifier}' image not found: '{path}'")]
    ImageNotFound {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`image` file does not exist")]
        span: SourceSpan,
        machine_identifier: String,
        path: String,
    },

    #[diagnostic(help(
        "Ensure the image path points to a file, not a directory or other non-file entry."
    ))]
    #[error("Machine '{machine_identifier}' image is not a file: '{path}'")]
    ImageIsNotAFile {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`image` must point to a file")]
        span: SourceSpan,
        machine_identifier: String,
        path: String,
    },

    #[diagnostic(help("Provide a non-empty username for the user entry."))]
    #[error("Machine '{machine_identifier}' has a user with an empty username")]
    EmptyUsername {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`username` must not be empty")]
        span: SourceSpan,
        machine_identifier: String,
    },

    #[diagnostic(help("Provide a non-empty password for the user entry."))]
    #[error("Machine '{machine_identifier}' has a user with an empty password")]
    EmptyPassword {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`password` must not be empty")]
        span: SourceSpan,
        machine_identifier: String,
    },

    #[diagnostic(help("Declare at least one `[[machines.users]]` entry for the machine."))]
    #[error("Machine '{machine_identifier}' must declare at least one user")]
    EmptyUsers {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("declare at least one user for this machine")]
        span: SourceSpan,
        machine_identifier: String,
    },

    #[diagnostic(help("Give each user a unique `username` within the machine configuration."))]
    #[error("Machine '{machine_identifier}' declares the username '{username}' more than once")]
    DuplicateUsername {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("duplicate `username` in this machine")]
        span: SourceSpan,
        machine_identifier: String,
        username: String,
    },

    #[diagnostic(help("Remove empty package names from the `install` list."))]
    #[error("Machine '{machine_identifier}' has an empty package name in `install`")]
    EmptyPackageName {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("package name must not be empty")]
        span: SourceSpan,
        machine_identifier: String,
    },

    #[diagnostic(help(
        "Use a compatible package manager for {distribution}: {compatible_managers}"
    ))]
    #[error(
        "Machine '{machine_identifier}' has incompatible package manager '{manager}' for {distribution}"
    )]
    IncompatibleManager {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("use a compatible manager for this operating system")]
        span: SourceSpan,
        machine_identifier: String,
        manager: String,
        distribution: String,
        compatible_managers: String,
    },

    #[diagnostic(help("Provide a non-empty path for the script entry."))]
    #[error("Machine '{machine_identifier}' has a script with an empty path")]
    EmptyScriptPath {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`path` must not be empty")]
        span: SourceSpan,
        machine_identifier: String,
    },

    #[diagnostic(help("Set `timeout_seconds` to a value between 1 and 3600 (inclusive)."))]
    #[error(
        "Machine '{machine_identifier}' has invalid script timeout: {actual} seconds (valid range: 1-3600)"
    )]
    InvalidScriptTimeout {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`timeout_seconds` must be between 1 and 3600")]
        span: SourceSpan,
        machine_identifier: String,
        actual: u32,
    },

    #[diagnostic(help("Ensure the script file exists at the specified path."))]
    #[error("Machine '{machine_identifier}' script not found: '{path}'")]
    ScriptNotFound {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`path` file does not exist")]
        span: SourceSpan,
        machine_identifier: String,
        path: String,
    },

    #[diagnostic(help(
        "Ensure the script path points to a file, not a directory or other non-file entry."
    ))]
    #[error("Machine '{machine_identifier}' script is not a file: '{path}'")]
    ScriptIsNotAFile {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("`path` must point to a file")]
        span: SourceSpan,
        machine_identifier: String,
        path: String,
    },

    #[diagnostic(help("Declare at least one `[[machines]]` entry in the configuration."))]
    #[error("Configuration must declare at least one machine")]
    EmptyMachines {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("add at least one machine entry")]
        span: SourceSpan,
    },

    #[diagnostic(help(
        "Give each machine a unique `identifier` within the laboratory configuration."
    ))]
    #[error(
        "Machine identifier '{identifier}' is declared more than once (machines at indices: {indices})"
    )]
    DuplicateMachineIdentifier {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("duplicate `identifier` in this laboratory")]
        span: SourceSpan,
        identifier: String,
        indices: String,
    },

    #[diagnostic(help(
        "Update the version field to match the current configuration version ({expected})."
    ))]
    #[error(
        "Configuration version mismatch: configuration uses {actual}, but MALINA expects version {expected}"
    )]
    FormatVersionMismatch {
        #[source_code]
        source_code: NamedSource<String>,
        #[label("version must be {expected}")]
        span: SourceSpan,
        actual: u64,
        expected: u64,
    },
}

impl Validation {
    pub fn from_toml_deserialize_error(
        error: DeserError,
        source_name: &str,
        source_code: &str,
    ) -> Self {
        let primary = error
            .errors
            .into_iter()
            .next()
            .expect("toml_span::DeserError always contains at least one error");

        Self::InvalidConfiguration {
            message: Self::format_toml_error(&primary),
            source_code: NamedSource::new(source_name, source_code.to_owned()),
            span: Self::highlighted_span(&primary),
        }
    }

    pub fn from_toml_error(error: toml_span::Error, source_name: &str, source_code: &str) -> Self {
        Self::InvalidConfiguration {
            message: Self::format_toml_error(&error),
            source_code: NamedSource::new(source_name, source_code.to_owned()),
            span: Self::highlighted_span(&error),
        }
    }

    fn highlighted_span(error: &toml_span::Error) -> Option<SourceSpan> {
        let span = match &error.kind {
            toml_span::ErrorKind::UnexpectedKeys { keys, .. } => {
                keys.first().map(|(_, span)| *span)
            }
            _ => Some(error.span),
        }?;

        (!span.is_empty()).then(|| to_source_span(span))
    }

    fn format_toml_error(error: &toml_span::Error) -> String {
        match &error.kind {
            toml_span::ErrorKind::UnexpectedKeys { keys, expected } => {
                let actual = format_quoted(keys.iter().map(|(name, _)| name.as_str()));
                let expected = format_quoted(expected.iter().map(String::as_str));

                if keys.len() == 1 {
                    format!("unknown field {actual}, expected one of {expected}")
                } else {
                    format!("unknown fields {actual}, expected one of {expected}")
                }
            }
            toml_span::ErrorKind::MissingField(field) => format!("missing field `{field}`"),
            toml_span::ErrorKind::UnexpectedValue { expected, value } => {
                let expected = format_quoted(expected.iter().copied());

                match value {
                    Some(value) => format!("unknown variant `{value}`, expected {expected}"),
                    None => format!("expected {expected}"),
                }
            }
            toml_span::ErrorKind::Wanted { expected, found } => {
                format!("found {found}, expected {expected}")
            }
            _ => error.to_string(),
        }
    }
}

pub fn format_quoted<'a>(items: impl IntoIterator<Item = &'a str>) -> String {
    let items = items
        .into_iter()
        .map(|item| format!("`{item}`"))
        .collect::<Vec<_>>();

    match items.as_slice() {
        [] => "(none)".to_owned(),
        [item] => item.clone(),
        [left, right] => format!("{left} or {right}"),
        _ => {
            let last = items.last().cloned().expect("items is not empty");

            format!("{} or {last}", items[..items.len() - 1].join(", "))
        }
    }
}

pub fn to_source_span(span: Span) -> SourceSpan {
    (span.start, span.end.saturating_sub(span.start)).into()
}
