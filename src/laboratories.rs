// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub use laboratory::Laboratory;
pub use machines::machine::Machine;
use serde::Deserialize;

mod laboratory;
pub mod machines;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub version: String,
    pub laboratory: Laboratory,
    pub machines: Vec<Machine>,
}
