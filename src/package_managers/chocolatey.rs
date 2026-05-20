// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    laboratories::machines::operating_system::OperatingSystem, package_managers::PackageManager,
};

#[derive(Debug, Clone, Copy)]
pub struct Chocolatey;

impl PackageManager for Chocolatey {
    fn as_str(&self) -> &'static str {
        "chocolatey"
    }

    fn install_package_manager_command(
        &self,
        operating_system: &OperatingSystem,
    ) -> Option<&'static str> {
        (operating_system.default_package_manager() != Some(self.as_str())).then_some(
            "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))",
        )
    }

    fn install_packages_command(&self, packages: &[&str]) -> Option<String> {
        (!packages.is_empty()).then(|| format!("choco install -y {}", packages.join(" ")))
    }
}
