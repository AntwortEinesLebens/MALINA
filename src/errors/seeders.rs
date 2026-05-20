// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use miette::Diagnostic;
use thiserror::Error;

use crate::errors::Template;

#[derive(Error, Diagnostic, Debug)]
pub enum Seeders {
    #[diagnostic(help("Ensure every declared script path points to a readable file."))]
    #[error("Failed to read declared script '{path}': {reason}")]
    ScriptReadFailed { path: String, reason: String },

    #[diagnostic(help("Fix the seed template before generating artifacts."))]
    #[error("Seed template rendering failed")]
    Template {
        #[source]
        source: Template,
    },

    #[diagnostic(help("Provide at least one package or fix the package manager implementation."))]
    #[error("Package manager '{manager}' could not generate a package install command")]
    PackageInstallCommandUnavailable { manager: &'static str },

    #[diagnostic(help("Regenerate the seed ISO before provider handoff."))]
    #[error("Failed to create seed ISO: {reason}")]
    IsoCreationFailed { reason: String },
}
