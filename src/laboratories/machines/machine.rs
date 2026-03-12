// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Machine {
    pub identifier: String,
    pub name: String,
    pub hardware: Hardware,
    pub operating_system: OperatingSystem,
}

#[derive(Debug, Deserialize)]
pub struct Hardware {
    pub cpus: u32,
    pub memory_megabyte: u32,
    pub disk_gigabyte: u32,
}

#[derive(Debug, Deserialize)]
pub struct OperatingSystem {
    pub family: String,
    pub version: String,
    pub image: String,
}
