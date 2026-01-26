// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Network {
    pub policy: Policy,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Policy {
    Keep,
    Remove,
}
