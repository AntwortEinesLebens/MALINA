// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use toml::Spanned;

#[derive(Debug, Deserialize)]
pub struct Machine {
    pub identifier: Spanned<String>,
    pub name: String,
    pub hardware: Hardware,
    pub operating_system: OperatingSystem,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hardware {
    pub cpus: Spanned<u32>,
    pub memory_megabyte: Spanned<u32>,
}

#[derive(Debug, Deserialize)]
pub struct OperatingSystem {
    pub family: String,
    pub version: String,
    pub image: String,
}
