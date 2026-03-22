// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::state::Machine;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum State {
    #[diagnostic(help(
        "Valid state transitions from '{current}': {valid_transitions}. Ensure the transition follows the machine lifecycle."
    ))]
    #[error("Invalid state transition from '{current}' to '{attempted}'")]
    InvalidTransition {
        current: Machine,
        attempted: Machine,
        valid_transitions: String,
    },

    #[diagnostic(help(
        "Valid states are: planned, provisioning, initialized, ready, failed. Use one of these exact values."
    ))]
    #[error("Unknown machine state: '{value}'")]
    UnknownState { value: String },
}
