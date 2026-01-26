// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use hardware::Hardware;
use operating_system::OperatingSystem;
use serde::Deserialize;
use std::path::PathBuf;

pub mod hardware;
pub mod operating_system;

#[derive(Deserialize)]
pub struct VirtualMachine {
    pub id: String,
    pub name: String,
    pub packages: Vec<PathBuf>,
    pub scripts: Vec<PathBuf>,
    pub hardware: Hardware,
    pub operating_system: OperatingSystem,
}
