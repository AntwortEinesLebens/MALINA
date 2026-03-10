// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub use crate::commands::Commands;
use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(name = "malina")]
#[command(author, version, about)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Increase output verbosity (can be used multiple times: -v, -vv, -vvv)
    #[arg(short, long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Enable quiet output (suppress non-essential messages)
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}
