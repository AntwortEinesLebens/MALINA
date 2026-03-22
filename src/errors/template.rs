// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Template {
    #[diagnostic(help("Check the embedded template registration and retry."))]
    #[error("Failed to register template '{name}': {reason}")]
    Registration { name: &'static str, reason: String },

    #[diagnostic(help("Check the template variables and retry the rendering operation."))]
    #[error("Failed to render template '{name}': {reason}")]
    Render { name: &'static str, reason: String },
}
