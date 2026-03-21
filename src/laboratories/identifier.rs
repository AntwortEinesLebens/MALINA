// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use regex::Regex;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    sync::OnceLock,
};
use toml_span::{DeserError, Deserialize, Error, ErrorKind, Value};

static KEBAB_CASE_REGEX: OnceLock<Regex> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(String);

impl Identifier {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parse(value: &str) -> Option<Self> {
        KEBAB_CASE_REGEX
            .get_or_init(|| Regex::new(r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$").unwrap())
            .is_match(value)
            .then(|| Self(value.to_owned()))
    }
}

impl Display for Identifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.0)
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let raw = value.take_string(None)?;

        Self::parse(raw.as_ref()).ok_or_else(|| {
            Error {
                kind: ErrorKind::UnexpectedValue {
                    expected: &["kebab-case identifier (lowercase letters, numbers, hyphens, starting with letter)"],
                    value: Some(raw.into_owned()),
                },
                span: value.span,
                line_info: None,
            }
            .into()
        })
    }
}
