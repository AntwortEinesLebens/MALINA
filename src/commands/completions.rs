// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::cli::Cli;
use clap::{CommandFactory, Subcommand};
use clap_complete::Shell;
use miette::Result;
use std::io;

#[derive(Subcommand, Debug)]
pub enum Completions {
    #[command(about = "Generate bash shell completion")]
    Bash,

    #[command(about = "Generate elvish shell completion")]
    Elvish,

    #[command(about = "Generate fish shell completion")]
    Fish,

    #[command(name = "powershell", about = "Generate powershell shell completion")]
    PowerShell,

    #[command(about = "Generate zsh shell completion")]
    Zsh,
}

pub fn execute(command: Completions) -> Result<()> {
    let shell = match command {
        Completions::Bash => Shell::Bash,
        Completions::Elvish => Shell::Elvish,
        Completions::Fish => Shell::Fish,
        Completions::PowerShell => Shell::PowerShell,
        Completions::Zsh => Shell::Zsh,
    };
    let mut cli = Cli::command();
    let name = cli.get_name().to_string();
    clap_complete::generate(shell, &mut cli, name, &mut io::stdout());

    Ok(())
}
