// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::cli::validate::Validate;
use clap::{Parser, Subcommand};
use miette::Result;

pub mod validate;

#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Validate(Validate),
}

impl Arguments {
    pub fn run(self) -> Result<()> {
        self.command.run()
    }
}

impl Command {
    pub fn run(self) -> Result<()> {
        match self {
            Command::Validate(command) => command.run(),
        }
    }
}
