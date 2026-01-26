// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use cli::Arguments;
use miette::Result;

mod cli;
mod laboratory;

fn main() -> Result<()> {
    Arguments::parse().run()
}
