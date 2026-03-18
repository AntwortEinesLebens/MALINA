// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::errors::{Validation, validation};
use miette::NamedSource;
use toml_span::{DeserError, Deserialize, Spanned, de_helpers::TableHelper};

#[derive(Debug)]
pub struct User {
    pub username: Spanned<String>,
    pub password: Spanned<String>,
}

impl<'de> Deserialize<'de> for User {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let username = table.required_s("username")?;
        let password = table.required_s("password")?;
        table.finalize(None)?;

        Ok(Self { username, password })
    }
}

impl User {
    pub fn validate(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
    ) -> Result<(), Validation> {
        self.validate_username(machine_identifier, source_name, source_code)?;
        self.validate_password(machine_identifier, source_name, source_code)?;

        Ok(())
    }

    fn validate_username(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
    ) -> Result<(), Validation> {
        if self.username.value.trim().is_empty() {
            return Err(Validation::EmptyUsername {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.username.span),
                machine_identifier: machine_identifier.to_owned(),
            });
        }

        Ok(())
    }

    fn validate_password(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
    ) -> Result<(), Validation> {
        if self.password.value.trim().is_empty() {
            return Err(Validation::EmptyPassword {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.password.span),
                machine_identifier: machine_identifier.to_owned(),
            });
        }

        Ok(())
    }
}
