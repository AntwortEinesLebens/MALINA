// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod completions;
pub mod deploy;
pub mod diagnose;
pub mod doctor;
pub mod laboratories;
pub mod validate;

pub use crate::commands::{completions::Completions, laboratories::Laboratories};
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Generate shell completion scripts")]
    #[command(subcommand)]
    Completions(Completions),

    #[command(about = "Validate a laboratory configuration file")]
    Validate {
        #[arg(help = "Path to the TOML configuration file")]
        path: PathBuf,
    },

    #[command(about = "Deploy a laboratory from configuration")]
    Deploy {
        #[arg(help = "Path to the TOML configuration file")]
        path: PathBuf,
    },

    #[command(about = "Check system readiness for laboratory deployment")]
    Doctor,

    #[command(about = "Diagnose failed or partial laboratory deployments")]
    Diagnose {
        #[arg(help = "Name of the laboratory to diagnose")]
        name: String,
    },

    #[command(about = "Manage deployed laboratories")]
    #[command(subcommand)]
    Laboratories(Laboratories),
}
