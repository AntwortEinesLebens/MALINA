// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    laboratories::machines::operating_system::OperatingSystem,
    package_managers::{PackageManager, shell_quote_join},
};

#[derive(Debug, Clone, Copy)]
pub struct Nix;

impl PackageManager for Nix {
    fn as_str(&self) -> &'static str {
        "nix"
    }

    fn install_package_manager_command(
        &self,
        operating_system: &OperatingSystem,
    ) -> Option<&'static str> {
        (operating_system.default_package_manager() != Some(self.as_str()))
            .then_some("curl -L https://nixos.org/nix/install | sh -s -- --daemon")
    }

    fn install_packages_command(&self, packages: &[&str]) -> Option<String> {
        (!packages.is_empty()).then(|| format!("nix install {}", shell_quote_join(packages)))
    }
}
