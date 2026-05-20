// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::laboratories::machines::{operating_system::OperatingSystem, packages::Manager};

mod apt;
mod chocolatey;
mod dnf;
mod nix;
mod winget;

pub use crate::package_managers::{
    apt::Apt, chocolatey::Chocolatey, dnf::Dnf, nix::Nix, winget::Winget,
};

pub trait PackageManager: Send + Sync {
    fn as_str(&self) -> &'static str;
    fn install_package_manager_command(
        &self,
        operating_system: &OperatingSystem,
    ) -> Option<&'static str>;
    fn install_packages_command(&self, packages: &[&str]) -> Option<String>;
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn shell_quote_join(values: &[&str]) -> String {
    values
        .iter()
        .map(|value| shell_quote(value))
        .collect::<Vec<_>>()
        .join(" ")
}

impl Manager {
    pub fn into_package_manager(self) -> Box<dyn PackageManager> {
        match self {
            Self::Apt => Box::new(Apt),
            Self::Dnf => Box::new(Dnf),
            Self::Nix => Box::new(Nix),
            Self::Winget => Box::new(Winget),
            Self::Chocolatey => Box::new(Chocolatey),
        }
    }
}
