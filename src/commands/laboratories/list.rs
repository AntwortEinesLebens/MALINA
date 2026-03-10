// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::logger::Logger;
use miette::Result;

pub fn execute() -> Result<()> {
    Logger::print("Listing laboratories");
    Logger::info("Scanning laboratory configurations");
    Logger::debug("Reading laboratory inventory");

    Ok(())
}
