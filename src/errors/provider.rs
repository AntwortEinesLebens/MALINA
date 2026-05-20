// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use miette::Diagnostic;
use thiserror::Error;

use crate::errors::Template;

#[derive(Error, Diagnostic, Debug)]
pub enum Provider {
    #[diagnostic(help("{remediation}"))]
    #[error("Provider backend '{provider}' is unavailable: {diagnostics}")]
    Unavailable {
        provider: String,
        diagnostics: String,
        remediation: String,
    },

    #[diagnostic(help("Ensure the machine identifier exists before retrying the operation."))]
    #[error("Provider machine not found: '{identifier}'")]
    MachineNotFound { identifier: String },

    #[diagnostic(help(
        "Use a unique machine identifier or destroy the existing domain before retrying."
    ))]
    #[error("Provider machine already exists: '{identifier}'")]
    MachineAlreadyExists { identifier: String },

    #[diagnostic(help(
        "This is an internal provider error. Please report this issue if it persists."
    ))]
    #[error("Internal provider error: {reason}")]
    Internal { reason: String },

    #[diagnostic(help("Generate the drive before retrying the attachment."))]
    #[error("Drive not found: '{path}'")]
    DriveNotFound { path: String },

    #[diagnostic(help("Provide a readable regular file before retrying the attachment."))]
    #[error("Drive is not a regular file: '{path}'")]
    DriveInvalid { path: String },

    #[diagnostic(help("{remediation}"))]
    #[error("Provider operation '{operation}' failed for machine '{identifier}': {reason}")]
    OperationFailed {
        operation: &'static str,
        identifier: String,
        reason: String,
        remediation: String,
    },

    #[diagnostic(help("Fix the provider template before retrying the provider operation."))]
    #[error("Provider template rendering failed")]
    Template {
        #[source]
        source: Template,
    },

    #[diagnostic(help(
        "Free up {resource_type} capacity or reduce the requested amount before retrying."
    ))]
    #[error(
        "Provider resource exhausted for '{resource_type}': required {required}, available {available}"
    )]
    ResourceExhausted {
        resource_type: &'static str,
        available: u64,
        required: u64,
    },
}
