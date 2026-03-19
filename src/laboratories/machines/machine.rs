// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    errors::{Validation, validation},
    laboratories::machines::{
        hardware::{Hardware, HostResources},
        operating_system::OperatingSystem,
        user::User,
    },
};
use miette::NamedSource;
use std::{collections::HashSet, path::Path};
use toml_span::{DeserError, Deserialize, Spanned, de_helpers::TableHelper};

#[derive(Debug)]
pub struct Machine {
    pub identifier: Spanned<String>,
    pub name: String,
    pub hardware: Hardware,
    pub operating_system: OperatingSystem,
    pub users: Spanned<Vec<User>>,
}

impl<'de> Deserialize<'de> for Machine {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let identifier = table.required_s("identifier")?;
        let name = table.required("name")?;
        let hardware = table.required("hardware")?;
        let operating_system = table.required("operating_system")?;
        let users = table.required_s::<Vec<User>>("users")?;
        table.finalize(None)?;

        Ok(Self {
            identifier,
            name,
            hardware,
            operating_system,
            users,
        })
    }
}

impl Machine {
    pub fn validate(
        &self,
        source_name: &str,
        source_code: &str,
        parent: &Path,
        host_resources: &HostResources,
    ) -> Result<(), Validation> {
        let identifier = self.identifier.value.as_str();

        self.hardware
            .validate(identifier, source_name, source_code, host_resources)?;
        self.operating_system
            .validate(identifier, source_name, source_code, parent)?;
        self.validate_users(identifier, source_name, source_code)?;

        Ok(())
    }

    fn validate_users(
        &self,
        identifier: &str,
        source_name: &str,
        source_code: &str,
    ) -> Result<(), Validation> {
        if self.users.value.is_empty() {
            return Err(Validation::EmptyUsers {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.users.span),
                machine_identifier: identifier.to_owned(),
            });
        }

        let mut seen_usernames = HashSet::with_capacity(self.users.value.len());

        for user in &self.users.value {
            user.validate(identifier, source_name, source_code)?;

            let normalized_username = user.username.value.trim();

            if !seen_usernames.insert(normalized_username) {
                return Err(Validation::DuplicateUsername {
                    source_code: NamedSource::new(source_name, source_code.to_owned()),
                    span: validation::to_source_span(user.username.span),
                    machine_identifier: identifier.to_owned(),
                    username: normalized_username.to_owned(),
                });
            }
        }

        Ok(())
    }
}
