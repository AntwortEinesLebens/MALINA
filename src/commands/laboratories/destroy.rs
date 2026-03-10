// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::logger::Logger;
use miette::Result;

pub fn execute(name: String) -> Result<()> {
    Logger::print(&format!("Destroying laboratory: {}", name));
    Logger::info(&format!("Laboratory: {}", name));
    Logger::debug("Removing virtual machines and resources");

    Ok(())
}
