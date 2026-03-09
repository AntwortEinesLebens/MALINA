// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::cli::Cli;
use crate::commands::{deploy, diagnose, doctor, laboratories, validate, Commands};
use clap::Parser;
use miette::Result;

mod cli;
mod commands;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { path } => validate::execute(path)?,
        Commands::Deploy { path } => deploy::execute(path)?,
        Commands::Doctor => doctor::execute()?,
        Commands::Diagnose { name } => diagnose::execute(name)?,
        Commands::Laboratories(subcommand) => laboratories::execute(subcommand)?,
    }

    Ok(())
}
