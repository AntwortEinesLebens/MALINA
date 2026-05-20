// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    errors::{Validation, validation},
    laboratories::machines::operating_system::OperatingSystem,
};
use miette::NamedSource;
use toml_span::{
    DeserError, Deserialize, Error, ErrorKind, Spanned, Value, de_helpers::TableHelper,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manager {
    Apt,
    Dnf,
    Nix,
    Winget,
    Chocolatey,
}

impl Manager {
    const EXPECTED_VALUES: &'static [&'static str] = &["apt", "dnf", "nix", "winget", "chocolatey"];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Apt => "apt",
            Self::Dnf => "dnf",
            Self::Nix => "nix",
            Self::Winget => "winget",
            Self::Chocolatey => "chocolatey",
        }
    }

    fn deserialize(manager: Spanned<String>) -> Result<Spanned<Self>, DeserError> {
        let value = match manager.value.as_str() {
            "apt" => Self::Apt,
            "dnf" => Self::Dnf,
            "nix" => Self::Nix,
            "winget" => Self::Winget,
            "chocolatey" => Self::Chocolatey,
            _ => {
                return Err(Error {
                    kind: ErrorKind::UnexpectedValue {
                        expected: Self::EXPECTED_VALUES,
                        value: Some(manager.value),
                    },
                    span: manager.span,
                    line_info: None,
                }
                .into());
            }
        };

        Ok(Spanned {
            value,
            span: manager.span,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Packages {
    pub manager: Spanned<Manager>,
    pub install: Spanned<Vec<Spanned<String>>>,
}

impl<'de> Deserialize<'de> for Packages {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let manager = Manager::deserialize(table.required_s("manager")?)?;
        let install = table.required_s::<Vec<Spanned<String>>>("install")?;
        table.finalize(None)?;

        Ok(Self { manager, install })
    }
}

impl Packages {
    pub fn validate(
        &self,
        operating_system: &OperatingSystem,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
    ) -> Result<(), Validation> {
        if self.install.value.is_empty() {
            return Err(Validation::EmptyPackageName {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.install.span),
                machine_identifier: machine_identifier.to_owned(),
            });
        }

        for package in &self.install.value {
            if package.value.trim().is_empty() {
                return Err(Validation::EmptyPackageName {
                    source_code: NamedSource::new(source_name, source_code.to_owned()),
                    span: validation::to_source_span(package.span),
                    machine_identifier: machine_identifier.to_owned(),
                });
            }

            if package.value.starts_with('-') {
                return Err(Validation::InvalidPackageName {
                    source_code: NamedSource::new(source_name, source_code.to_owned()),
                    span: validation::to_source_span(package.span),
                    machine_identifier: machine_identifier.to_owned(),
                    package: package.value.clone(),
                });
            }
        }

        let package_manager = self.manager.value.into_package_manager();

        if !operating_system.supports_package_manager(package_manager.as_ref()) {
            return Err(Validation::IncompatibleManager {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(self.manager.span),
                machine_identifier: machine_identifier.to_owned(),
                manager: self.manager.value.as_str().to_owned(),
                distribution: operating_system.name().to_owned(),
                compatible_managers: operating_system.compatible_package_managers_display(),
            });
        }

        Ok(())
    }
}
