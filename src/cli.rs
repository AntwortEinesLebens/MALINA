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
    #[arg(short, long, action = ArgAction::Count, global = true, help = "Increase output verbosity (can be used multiple times: -v, -vv, -vvv)")]
    pub verbose: u8,

    #[arg(
        short,
        long,
        global = true,
        conflicts_with = "verbose",
        help = "Enable quiet output (suppress non-essential messages)"
    )]
    pub quiet: bool,

    #[arg(long, global = true, help = "Disable colored output")]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}
