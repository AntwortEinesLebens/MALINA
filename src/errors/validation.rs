// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use miette::{Diagnostic, NamedSource, SourceSpan};
use std::io::Error as IoError;
use thiserror::Error;
use toml_span::{DeserError, Error as TomlError, ErrorKind as TomlErrorKind, Span};

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
            source_code: NamedSource::new(source_name.to_owned(), source_code.to_owned()),
            span: Self::highlighted_span(&primary),
        }
    }

    pub fn from_toml_error(error: TomlError, source_name: &str, source_code: &str) -> Self {
        Self::InvalidConfiguration {
            message: Self::format_toml_error(&error),
            source_code: NamedSource::new(source_name.to_owned(), source_code.to_owned()),
            span: Self::highlighted_span(&error),
        }
    }

    fn highlighted_span(error: &TomlError) -> Option<SourceSpan> {
        let span = match &error.kind {
            TomlErrorKind::UnexpectedKeys { keys, .. } => keys.first().map(|(_, span)| *span),
            _ => Some(error.span),
        }?;

        (!span.is_empty()).then(|| to_source_span(span))
    }

    fn format_toml_error(error: &TomlError) -> String {
        match &error.kind {
            TomlErrorKind::UnexpectedKeys { keys, expected } => {
                let actual = Self::format_quoted(keys.iter().map(|(name, _)| name.as_str()));
                let expected = Self::format_quoted(expected.iter().map(String::as_str));

                if keys.len() == 1 {
                    format!("unknown field {actual}, expected one of {expected}")
                } else {
                    format!("unknown fields {actual}, expected one of {expected}")
                }
            }
            TomlErrorKind::MissingField(field) => format!("missing field `{field}`"),
            TomlErrorKind::UnexpectedValue { expected, value } => {
                let expected = Self::format_quoted(expected.iter().copied());

                match value {
                    Some(value) => format!("unknown variant `{value}`, expected {expected}"),
                    None => format!("expected {expected}"),
                }
            }
            TomlErrorKind::Wanted { expected, found } => {
                format!("invalid type: {found}, expected {expected}")
            }
            _ => error.to_string(),
        }
    }

    fn format_quoted<'a>(items: impl IntoIterator<Item = &'a str>) -> String {
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
}

pub fn to_source_span(span: Span) -> SourceSpan {
    (span.start, span.end.saturating_sub(span.start)).into()
}
