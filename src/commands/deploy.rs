// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use miette::Result;
use std::path::PathBuf;

pub fn execute(path: PathBuf) -> Result<()> {
    println!("Deploying laboratory from: {}", path.display());

    Ok(())
}
