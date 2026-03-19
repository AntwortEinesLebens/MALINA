// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    errors::{Validation, format_quoted, validation},
    laboratories::machines::packages::Manager,
};
use miette::NamedSource;
use std::path::{Path, PathBuf};
use toml_span::{
    DeserError, Deserialize, Error, ErrorKind, Spanned, Value, de_helpers::TableHelper,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebianVersion {
    Debian13,
}

impl DebianVersion {
    const EXPECTED_VALUES: &'static [&'static str] = &["13"];

    fn deserialize(version: Spanned<String>) -> Result<Self, DeserError> {
        match version.value.as_str() {
            "13" => Ok(Self::Debian13),
            _ => Err(Error {
                kind: ErrorKind::UnexpectedValue {
                    expected: Self::EXPECTED_VALUES,
                    value: Some(version.value),
                },
                span: version.span,
                line_info: None,
            }
            .into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FedoraVersion {
    Fedora43,
}

impl FedoraVersion {
    const EXPECTED_VALUES: &'static [&'static str] = &["43"];

    fn deserialize(version: Spanned<String>) -> Result<Self, DeserError> {
        match version.value.as_str() {
            "43" => Ok(Self::Fedora43),
            _ => Err(Error {
                kind: ErrorKind::UnexpectedValue {
                    expected: Self::EXPECTED_VALUES,
                    value: Some(version.value),
                },
                span: version.span,
                line_info: None,
            }
            .into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsVersion {
    Windows11,
}

impl WindowsVersion {
    const EXPECTED_VALUES: &'static [&'static str] = &["11"];

    fn deserialize(version: Spanned<String>) -> Result<Self, DeserError> {
        match version.value.as_str() {
            "11" => Ok(Self::Windows11),
            _ => Err(Error {
                kind: ErrorKind::UnexpectedValue {
                    expected: Self::EXPECTED_VALUES,
                    value: Some(version.value),
                },
                span: version.span,
                line_info: None,
            }
            .into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxDistribution {
    Debian(DebianVersion),
    Fedora(FedoraVersion),
}

impl LinuxDistribution {
    const EXPECTED_VALUES: &'static [&'static str] = &["debian", "fedora"];

    fn deserialize(
        distribution: Spanned<String>,
        version: Spanned<String>,
    ) -> Result<Self, DeserError> {
        match distribution.value.as_str() {
            "debian" => DebianVersion::deserialize(version).map(Self::Debian),
            "fedora" => FedoraVersion::deserialize(version).map(Self::Fedora),
            _ => Err(Error {
                kind: ErrorKind::UnexpectedValue {
                    expected: Self::EXPECTED_VALUES,
                    value: Some(distribution.value),
                },
                span: distribution.span,
                line_info: None,
            }
            .into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperatingSystem {
    Linux {
        distribution: LinuxDistribution,
        image: Spanned<PathBuf>,
    },
    Windows {
        version: WindowsVersion,
        image: Spanned<PathBuf>,
    },
}

impl OperatingSystem {
    const EXPECTED_FAMILIES: &'static [&'static str] = &["linux", "windows"];
    const SUPPORTED_IMAGE_EXTENSIONS: &'static [&'static str] = &["qcow2"];

    fn deserialize_linux(mut table: TableHelper<'_>) -> Result<Self, DeserError> {
        let distribution = table.required_s::<String>("distribution")?;
        let version = table.required_s::<String>("version")?;
        let image: Spanned<PathBuf> = table.required_s::<String>("image")?.map();
        table.finalize(None)?;

        Ok(Self::Linux {
            distribution: LinuxDistribution::deserialize(distribution, version)?,
            image,
        })
    }

    fn deserialize_windows(mut table: TableHelper<'_>) -> Result<Self, DeserError> {
        let version = WindowsVersion::deserialize(table.required_s::<String>("version")?)?;
        let image: Spanned<PathBuf> = table.required_s::<String>("image")?.map();
        table.finalize(None)?;

        Ok(Self::Windows { version, image })
    }

    pub fn distribution_name(&self) -> &'static str {
        match self {
            Self::Linux {
                distribution: LinuxDistribution::Debian(_),
                ..
            } => "debian",
            Self::Linux {
                distribution: LinuxDistribution::Fedora(_),
                ..
            } => "fedora",
            Self::Windows { .. } => "windows",
        }
    }

    pub fn compatible_managers(&self) -> &'static [Manager] {
        match self {
            Self::Linux {
                distribution: LinuxDistribution::Debian(_),
                ..
            } => &[Manager::Apt, Manager::Nix],
            Self::Linux {
                distribution: LinuxDistribution::Fedora(_),
                ..
            } => &[Manager::Dnf, Manager::Nix],
            Self::Windows { .. } => &[Manager::Winget, Manager::Chocolatey],
        }
    }

    pub fn compatible_managers_display(&self) -> String {
        format_quoted(self.compatible_managers().iter().map(Manager::as_str))
    }

    pub fn validate(
        &self,
        machine_identifier: &str,
        source_name: &str,
        source_code: &str,
        parent: &Path,
    ) -> Result<(), Validation> {
        let image = match self {
            Self::Linux { image, .. } | Self::Windows { image, .. } => image,
        };
        let path = if image.value.is_absolute() {
            image.value.clone()
        } else {
            parent.join(&image.value)
        };

        let extension = match image
            .value
            .extension()
            .and_then(|extension| extension.to_str())
        {
            Some(extension) => extension,
            None => {
                return Err(Validation::InvalidImageExtension {
                    source_code: NamedSource::new(source_name, source_code.to_owned()),
                    span: validation::to_source_span(image.span),
                    machine_identifier: machine_identifier.to_owned(),
                    actual: "(no extension)".to_owned(),
                });
            }
        };

        if !Self::SUPPORTED_IMAGE_EXTENSIONS.contains(&extension) {
            return Err(Validation::InvalidImageExtension {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(image.span),
                machine_identifier: machine_identifier.to_owned(),
                actual: extension.to_owned(),
            });
        }

        if !path.exists() {
            return Err(Validation::ImageNotFound {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(image.span),
                machine_identifier: machine_identifier.to_owned(),
                path: path.display().to_string(),
            });
        }

        if !path.is_file() {
            return Err(Validation::ImageIsNotAFile {
                source_code: NamedSource::new(source_name, source_code.to_owned()),
                span: validation::to_source_span(image.span),
                machine_identifier: machine_identifier.to_owned(),
                path: path.display().to_string(),
            });
        }

        Ok(())
    }
}

impl<'de> Deserialize<'de> for OperatingSystem {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let family = table.required_s::<String>("family")?;

        match family.value.as_str() {
            "linux" => Self::deserialize_linux(table),
            "windows" => Self::deserialize_windows(table),
            _ => Err(Error {
                kind: ErrorKind::UnexpectedValue {
                    expected: OperatingSystem::EXPECTED_FAMILIES,
                    value: Some(family.value),
                },
                span: family.span,
                line_info: None,
            }
            .into()),
        }
    }
}
