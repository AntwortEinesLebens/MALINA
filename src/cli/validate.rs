// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::laboratory::Laboratory;
use clap::Args;
use miette::{Context, IntoDiagnostic, Result};
use std::{fs, path::PathBuf};

#[derive(Args)]
pub struct Validate {
    #[arg(required = true, help = "Path to the laboratory configuration file")]
    path: PathBuf,
}

impl Validate {
    pub fn run(self) -> Result<()> {
        let raw = fs::read_to_string(&self.path)
            .into_diagnostic()
            .with_context(|| format!("failed to read configuration at {}", self.path.display()))?;

        toml::from_str::<Laboratory>(&raw)
            .into_diagnostic()
            .with_context(|| format!("failed to parse {}", self.path.display()))?;

        println!(
            "{} is a valid laboratory configuration file.",
            self.path.display()
        );

        Ok(())
    }
}
