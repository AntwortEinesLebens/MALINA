// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::laboratories::machines::machine::Machine;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub version: String,
    pub laboratory: Laboratory,
    pub machines: Vec<Machine>,
}

#[derive(Debug, Deserialize)]
pub struct Laboratory {
    pub name: String,
}
