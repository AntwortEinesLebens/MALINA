// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    cli::Cli,
    commands::{completions, deploy, diagnose, doctor, laboratories, validate, Commands},
    logger::Logger,
};
use clap::Parser;
use miette::{MietteHandlerOpts, Result};

mod cli;
mod commands;
mod errors;
mod logger;

fn main() -> Result<()> {
    let cli = Cli::parse();

    Logger::initialize(cli.quiet, cli.verbose, cli.no_color);

    miette::set_hook(Box::new(move |_| {
        let mut handler = MietteHandlerOpts::new();

        if cli.no_color {
            handler = handler.color(false);
        }

        Box::new(handler.build())
    }))
    .map_err(|error| miette::miette!("Failed to initialize miette error handler: {}", error))?;

    match cli.command {
        Commands::Completions(subcommand) => completions::execute(subcommand)?,
        Commands::Validate { path } => validate::execute(path)?,
        Commands::Deploy { path } => deploy::execute(path)?,
        Commands::Doctor => doctor::execute()?,
        Commands::Diagnose { name } => diagnose::execute(name)?,
        Commands::Laboratories(subcommand) => laboratories::execute(subcommand)?,
    }

    Ok(())
}
