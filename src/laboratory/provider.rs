// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Provider {
    pub r#type: Type,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Kvm,
}
