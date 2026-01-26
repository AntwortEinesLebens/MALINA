// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Deserializer};
use std::path::PathBuf;

pub struct OperatingSystem {
    pub family: Family,
    pub image: PathBuf,
    pub details: Details,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Family {
    Linux,
    Windows,
}

#[derive(Deserialize)]
#[serde(tag = "family", rename_all = "snake_case")]
pub enum Details {
    Linux { distribution: LinuxDistribution },
    Windows { version: WindowsVersion },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinuxDistribution {
    Debian,
    Ubuntu,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowsVersion {
    #[serde(rename = "11")]
    Windows11,
    #[serde(rename = "10")]
    Windows10,
    #[serde(rename = "8.1")]
    Windows8_1,
    #[serde(rename = "8")]
    Windows8,
}

impl From<&Details> for Family {
    fn from(details: &Details) -> Self {
        match details {
            Details::Linux { .. } => Family::Linux,
            Details::Windows { .. } => Family::Windows,
        }
    }
}

impl<'de> Deserialize<'de> for OperatingSystem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(flatten)]
            details: Details,
            image: PathBuf,
        }

        let raw = Raw::deserialize(deserializer)?;

        Ok(Self {
            family: Family::from(&raw.details),
            image: raw.image,
            details: raw.details,
        })
    }
}
