// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use toml_span::{DeserError, Deserialize, de_helpers::TableHelper};

#[derive(Debug)]
pub struct Laboratory {
    pub name: String,
}

impl<'de> Deserialize<'de> for Laboratory {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let name = table.required("name")?;
        table.finalize(None)?;

        Ok(Self { name })
    }
}
