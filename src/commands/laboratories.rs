// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod destroy;
pub mod list;
pub mod start;
pub mod stop;

use clap::Subcommand;
use miette::Result;

#[derive(Subcommand, Debug)]
pub enum Laboratories {
    #[command(about = "List all deployed laboratories")]
    List,

    #[command(about = "Start a stopped laboratory")]
    Start {
        #[arg(help = "Name of the laboratory to start")]
        name: String,
    },

    #[command(about = "Stop a running laboratory")]
    Stop {
        #[arg(help = "Name of the laboratory to stop")]
        name: String,
    },

    #[command(about = "Destroy a laboratory")]
    Destroy {
        #[arg(help = "Name of the laboratory to destroy")]
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
