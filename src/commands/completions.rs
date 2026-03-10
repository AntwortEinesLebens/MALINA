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
    /// Generate bash shell completion
    Bash,

    /// Generate elvish shell completion
    Elvish,

    /// Generate fish shell completion
    Fish,

    /// Generate powershell shell completion
    #[command(name = "powershell")]
    PowerShell,

    /// Generate zsh shell completion
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
