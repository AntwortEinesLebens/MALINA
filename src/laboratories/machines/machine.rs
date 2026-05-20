// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    errors::{Validation, validation},
    laboratories::{
        identifier::Identifier,
        laboratory::Provider,
        machines::{
            hardware::{Hardware, HostResources},
            operating_system::OperatingSystem,
            packages::Packages,
            script::Script,
            user::User,
        },
    },
    logger::Logger,
};
use miette::NamedSource;
use std::{collections::HashSet, path::Path};
use toml_span::{DeserError, Deserialize, Spanned, de_helpers::TableHelper};

#[derive(Debug, Clone)]
pub struct Machine {
    pub identifier: Spanned<Identifier>,
    pub name: String,
    pub hardware: Hardware,
    pub operating_system: OperatingSystem,
    pub users: Spanned<Vec<User>>,
    pub packages: Option<Packages>,
    pub scripts: Option<Vec<Script>>,
}

impl<'de> Deserialize<'de> for Machine {
    fn deserialize(value: &mut toml_span::Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let identifier = table.required_s("identifier")?;
        let name = table.required("name")?;
        let hardware = table.required("hardware")?;
        let operating_system = table.required("operating_system")?;
        let users = table.required_s::<Vec<User>>("users")?;
        let packages = table.optional("packages");
        let scripts = table.optional("scripts");
        table.finalize(None)?;

        Ok(Self {
            identifier,
            name,
            hardware,
            operating_system,
            users,
            packages,
            scripts,
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
        provider: Provider,
    ) -> Result<(), Validation> {
        let identifier = self.identifier.value.as_str();

        Logger::info("Checking hardware");

        self.hardware
            .validate(identifier, source_name, source_code, host_resources)?;

        Logger::info("Checking operating system");

        self.operating_system.validate(
            identifier,
            source_name,
            source_code,
            parent,
            provider.supported_image_extensions(),
        )?;

        Logger::info(&format!("Checking users ({})", self.users.value.len()));

        self.validate_users(identifier, source_name, source_code)?;

        if let Some(packages) = &self.packages {
            Logger::info(&format!(
                "Checking packages ({})",
                packages.install.value.len()
            ));

            packages.validate(&self.operating_system, identifier, source_name, source_code)?;
        }

        if let Some(scripts) = &self.scripts {
            Logger::info(&format!("Checking scripts ({})", scripts.len()));

            let mut seen_filenames = HashSet::with_capacity(scripts.len());

            for script in scripts {
                script.validate(identifier, source_name, source_code, parent)?;
                self.validate_windows_script_extension(
                    script,
                    identifier,
                    source_name,
                    source_code,
                )?;

                let filename = script
                    .path
                    .value
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("")
                    .trim();

                if !seen_filenames.insert(filename.to_owned()) {
                    return Err(Validation::DuplicateScriptFilename {
                        source_code: NamedSource::new(source_name, source_code.to_owned()),
                        span: validation::to_source_span(script.path.span),
                        machine_identifier: identifier.to_owned(),
                        filename: filename.to_owned(),
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_windows_script_extension(
        &self,
        script: &Script,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
    ) -> Result<(), Validation> {
        if !matches!(self.operating_system, OperatingSystem::Windows { .. }) {
            return Ok(());
        }

        let extension = script
            .path
            .value
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("");

        if extension != "ps1" {
            return Err(Validation::InvalidWindowsScriptExtension {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(script.path.span),
                machine_identifier: machine_identifier.to_owned(),
                actual: extension.to_owned(),
            });
        }

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
