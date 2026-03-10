// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Subcommand;
use miette::Result;

pub mod destroy;
pub mod list;
pub mod start;
pub mod stop;

#[derive(Subcommand, Debug)]
pub enum Laboratories {
    /// List all deployed laboratories
    List,

    /// Start a stopped laboratory
    Start {
        /// Name of the laboratory to start
        name: String,
    },

    /// Stop a running laboratory
    Stop {
        /// Name of the laboratory to stop
        name: String,
    },

    /// Destroy a laboratory
    Destroy {
        /// Name of the laboratory to destroy
        name: String,
    },
}

pub fn execute(command: Laboratories) -> Result<()> {
    match command {
        Laboratories::List => list::execute(),
        Laboratories::Start { name } => start::execute(name),
        Laboratories::Stop { name } => stop::execute(name),
        Laboratories::Destroy { name } => destroy::execute(name),
    }
}
