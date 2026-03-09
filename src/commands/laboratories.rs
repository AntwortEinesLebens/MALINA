// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::commands::Laboratories;
use miette::Result;

pub mod destroy;
pub mod list;
pub mod start;
pub mod stop;

pub fn execute(command: Laboratories) -> Result<()> {
    match command {
        Laboratories::List => list::execute(),
        Laboratories::Start { name } => start::execute(name),
        Laboratories::Stop { name } => stop::execute(name),
        Laboratories::Destroy { name } => destroy::execute(name),
    }
}
