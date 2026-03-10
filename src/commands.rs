// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Subcommand;
pub use laboratories::Laboratories;
use std::path::PathBuf;

pub mod deploy;
pub mod diagnose;
pub mod doctor;
pub mod laboratories;
pub mod validate;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Validate a laboratory configuration file
    Validate {
        /// Path to the TOML configuration file
        path: PathBuf,
    },

    /// Deploy a laboratory from configuration
    Deploy {
        /// Path to the TOML configuration file
        path: PathBuf,
    },

    /// Check system readiness for laboratory deployment
    Doctor,

    /// Diagnose failed or partial laboratory deployments
    Diagnose {
        /// Name of the laboratory to diagnose
        name: String,
    },

    /// Manage deployed laboratories
    #[command(subcommand)]
    Laboratories(Laboratories),
}
