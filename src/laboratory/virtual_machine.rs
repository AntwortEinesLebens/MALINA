// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use std::path::PathBuf;

pub mod hardware;
pub mod operating_system;

use hardware::Hardware;
use operating_system::OperatingSystem;

#[derive(Deserialize)]
pub struct VirtualMachine {
    pub id: String,
    pub name: String,
    pub packages: Vec<PathBuf>,
    pub scripts: Vec<PathBuf>,
    pub hardware: Hardware,
    pub operating_system: OperatingSystem,
}
