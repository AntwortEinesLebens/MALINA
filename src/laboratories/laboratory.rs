// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use toml_span::{DeserError, Deserialize, Error, ErrorKind, Value, de_helpers::TableHelper};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Isolated,
    Preserved,
}

impl Network {
    const EXPECTED_VALUES: &'static [&'static str] = &["isolated", "preserved"];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Isolated => "isolated",
            Self::Preserved => "preserved",
        }
    }
}

impl<'de> Deserialize<'de> for Network {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let string_value = value.take_string(None)?;
        match string_value.as_ref() {
            "isolated" => Ok(Self::Isolated),
            "preserved" => Ok(Self::Preserved),
            _ => Err(Error {
                kind: ErrorKind::UnexpectedValue {
                    expected: Self::EXPECTED_VALUES,
                    value: Some(string_value.into_owned()),
                },
                span: value.span,
                line_info: None,
            }
            .into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Kvm,
}

impl Provider {
    const EXPECTED_VALUES: &'static [&'static str] = &["kvm"];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Kvm => "kvm",
        }
    }
}

impl<'de> Deserialize<'de> for Provider {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let string_value = value.take_string(None)?;
        match string_value.as_ref() {
            "kvm" => Ok(Self::Kvm),
            _ => Err(Error {
                kind: ErrorKind::UnexpectedValue {
                    expected: Self::EXPECTED_VALUES,
                    value: Some(string_value.into_owned()),
                },
                span: value.span,
                line_info: None,
            }
            .into()),
        }
    }
}

#[derive(Debug)]
pub struct Laboratory {
    pub name: String,
    pub network: Network,
    pub provider: Provider,
}

impl<'de> Deserialize<'de> for Laboratory {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let name = table.required("name")?;
        let network = table.required("network")?;
        let provider = table.required("provider")?;
        table.finalize(None)?;

        Ok(Self {
            name,
            network,
            provider,
        })
    }
}
