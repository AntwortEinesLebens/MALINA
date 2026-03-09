// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub use crate::commands::Commands;
use clap::Parser;

/// MALINA - Malware Analysis Laboratory Infrastructure for Networked Isolated Architectures
#[derive(Parser, Debug)]
#[command(name = "malina")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Enable quiet output (suppress non-essential messages)
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}
