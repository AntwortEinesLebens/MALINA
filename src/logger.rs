// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use env_logger::{Builder, WriteStyle};
use log::{Level, LevelFilter};
use owo_colors::OwoColorize;
use std::{io::Write, sync::OnceLock};

static QUIET_MODE: OnceLock<bool> = OnceLock::new();

pub struct Logger;

impl Logger {
    pub fn initialize(quiet: bool, verbose: u8, no_color: bool) {
        let _ = QUIET_MODE.set(quiet);

        if no_color {
            owo_colors::colored::control::set_override(false);
        }

        let level = match verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        let level = if quiet { LevelFilter::Off } else { level };

        Builder::new()
            .filter_level(level)
            .write_style(if no_color {
                WriteStyle::Never
            } else {
                WriteStyle::Always
            })
            .format(|formatter, record| {
                let level = record.level();
                let message = record.args();

                match level {
                    Level::Error => writeln!(formatter, "{} {}", "ERROR".red().bold(), message),
                    Level::Warn => writeln!(formatter, "{} {}", "WARN".yellow().bold(), message),
                    Level::Info => writeln!(formatter, "{} {}", "INFO".cyan().bold(), message),
                    Level::Debug => writeln!(formatter, "{} {}", "DEBUG".blue().bold(), message),
                    Level::Trace => writeln!(formatter, "{} {}", "TRACE".magenta().bold(), message),
                }
            })
            .init();
    }

    pub fn print(message: &str) {
        if !QUIET_MODE.get().unwrap_or(&false) {
            println!("{}", message);
        }
    }

    pub fn info(message: &str) {
        log::info!("{}", message);
    }

    pub fn debug(message: &str) {
        log::debug!("{}", message);
    }

    pub fn warn(message: &str) {
        log::warn!("{}", message);
    }
}
