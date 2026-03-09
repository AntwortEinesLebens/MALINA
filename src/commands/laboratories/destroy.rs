// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use miette::Result;

pub fn execute(name: String) -> Result<()> {
    println!("Destroying laboratory: {}", name);

    Ok(())
}
