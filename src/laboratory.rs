// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use network::Network;
use provider::Provider;
use serde::Deserialize;
use virtual_machine::VirtualMachine;

pub mod network;
pub mod provider;
pub mod virtual_machine;

#[derive(Deserialize)]
pub struct Laboratory {
    pub version: u16,
    pub provider: Provider,
    pub network: Network,
    pub virtual_machines: Vec<VirtualMachine>,
}
