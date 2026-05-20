// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    laboratories::machines::operating_system::OperatingSystem, package_managers::PackageManager,
};

#[derive(Debug, Clone, Copy)]
pub struct Winget;

impl PackageManager for Winget {
    fn as_str(&self) -> &'static str {
        "winget"
    }

    fn install_package_manager_command(
        &self,
        _operating_system: &OperatingSystem,
    ) -> Option<&'static str> {
        None
    }

    fn install_packages_command(&self, packages: &[&str]) -> Option<String> {
        (!packages.is_empty()).then(|| {
            packages
                .iter()
                .map(|package| {
                    format!(
                        "winget install --exact --id {} --accept-package-agreements --accept-source-agreements --disable-interactivity",
                        package
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
    }
}
