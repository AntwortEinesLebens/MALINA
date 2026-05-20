// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod cloud_init;
pub mod cloudbase_init;

pub use crate::seeders::{cloud_init::CloudInit, cloudbase_init::CloudbaseInit};
use crate::{errors::Seeders, laboratories::Machine};
use std::path::Path;

pub trait Seeder {
    fn name(&self) -> &'static str;
    fn generate_iso(&self, machine: &Machine, parent: &Path) -> Result<Vec<u8>, Seeders>;
}
