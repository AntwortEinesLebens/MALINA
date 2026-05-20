// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    laboratories::machines::operating_system::OperatingSystem,
    package_managers::{PackageManager, shell_quote_join},
};

#[derive(Debug, Clone, Copy)]
pub struct Apt;

impl PackageManager for Apt {
    fn as_str(&self) -> &'static str {
        "apt"
    }

    fn install_package_manager_command(
        &self,
        operating_system: &OperatingSystem,
    ) -> Option<&'static str> {
        (operating_system.default_package_manager() != Some(self.as_str())).then_some(
            "export DEBIAN_FRONTEND=noninteractive\napt-get -yq update\napt-get -yq install apt",
        )
    }

    fn install_packages_command(&self, packages: &[&str]) -> Option<String> {
        (!packages.is_empty()).then(|| {
            format!(
                "export DEBIAN_FRONTEND=noninteractive\napt-get -o Dpkg::Options::=--force-confdef -o Dpkg::Options::=--force-confold -yq update\napt-get -o Dpkg::Options::=--force-confdef -o Dpkg::Options::=--force-confold -yq install {}",
                shell_quote_join(packages)
            )
        })
    }
}
