// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::errors::{Validation, validation};
use miette::NamedSource;
use std::path::{Path, PathBuf};
use toml_span::{
    DeserError, Deserialize, Error, ErrorKind, Spanned, Value, de_helpers::TableHelper,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnFailure {
    Warn,
    Fail,
}

impl OnFailure {
    const EXPECTED_VALUES: &'static [&'static str] = &["warn", "fail"];

    fn deserialize(on_failure: Spanned<String>) -> Result<Spanned<Self>, DeserError> {
        let value = match on_failure.value.as_str() {
            "warn" => Self::Warn,
            "fail" => Self::Fail,
            _ => {
                return Err(Error {
                    kind: ErrorKind::UnexpectedValue {
                        expected: Self::EXPECTED_VALUES,
                        value: Some(on_failure.value),
                    },
                    span: on_failure.span,
                    line_info: None,
                }
                .into());
            }
        };

        Ok(Spanned {
            value,
            span: on_failure.span,
        })
    }
}

#[derive(Debug)]
pub struct Script {
    pub path: Spanned<PathBuf>,
    pub timeout_seconds: Option<Spanned<u32>>,
    pub on_failure: Option<Spanned<OnFailure>>,
}

impl<'de> Deserialize<'de> for Script {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let path: Spanned<PathBuf> = table.required_s::<String>("path")?.map();
        let timeout_seconds = table.optional_s("timeout_seconds");
        let on_failure = table
            .optional_s::<String>("on_failure")
            .map(OnFailure::deserialize)
            .transpose()?;
        table.finalize(None)?;

        Ok(Self {
            path,
            timeout_seconds,
            on_failure,
        })
    }
}

impl Script {
    pub fn validate(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
        parent: &Path,
    ) -> Result<(), Validation> {
        self.validate_path(machine_identifier, source_name, source_code, parent)?;
        self.validate_timeout(machine_identifier, source_name, source_code)?;

        Ok(())
    }

    fn validate_path(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
        parent: &Path,
    ) -> Result<(), Validation> {
        if self
            .path
            .value
            .as_os_str()
            .to_string_lossy()
            .trim()
            .is_empty()
        {
            return Err(Validation::EmptyScriptPath {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.path.span),
                machine_identifier: machine_identifier.to_owned(),
            });
        }

        let path = if self.path.value.is_absolute() {
            self.path.value.clone()
        } else {
            parent.join(&self.path.value)
        };

        if !path.exists() {
            return Err(Validation::ScriptNotFound {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.path.span),
                machine_identifier: machine_identifier.to_owned(),
                path: path.display().to_string(),
            });
        }

        if !path.is_file() {
            return Err(Validation::ScriptIsNotAFile {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.path.span),
                machine_identifier: machine_identifier.to_owned(),
                path: path.display().to_string(),
            });
        }

        Ok(())
    }

    fn validate_timeout(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
    ) -> Result<(), Validation> {
        if let Some(timeout) = &self.timeout_seconds {
            if timeout.value < 1 || timeout.value > 3600 {
                return Err(Validation::InvalidScriptTimeout {
                    source_code: NamedSource::new(source_name, source_code.to_owned()),
                    span: validation::to_source_span(timeout.span),
                    machine_identifier: machine_identifier.to_owned(),
                    actual: timeout.value,
                });
            }
        }

        Ok(())
    }
}
