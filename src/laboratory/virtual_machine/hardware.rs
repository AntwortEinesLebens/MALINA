// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Hardware {
    pub cpus: u16,
    pub memory_megabyte: u32,
    pub disk_gigabyte: u32,
}
