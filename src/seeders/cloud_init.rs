// SPDX-FileCopyrightText: 2025 The MALINA development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{errors::Seeders, laboratories::Machine, seeders::Seeder, template::Engine};
use hadris_iso::{
    read::PathSeparator,
    write::{File as IsoFile, InputFiles, IsoImageWriter, estimator, options},
};
use std::{io::Cursor, path::Path, sync::Arc};
use tera::Context;

#[derive(Debug, Clone, Copy)]
pub struct CloudInit;

impl Seeder for CloudInit {
    fn name(&self) -> &'static str {
        "cloud-init"
    }

    fn generate_iso(&self, machine: &Machine, parent: &Path) -> Result<Vec<u8>, Seeders> {
        let meta_data = self.render_meta_data(machine)?;
        let scripts = self.render_script_context(machine, parent)?;
        let script_manifest = self.render_script_manifest(&scripts);
        let user_data = self.render_user_data(machine, !scripts.is_empty())?;
        let network_config: Option<String> = None;

        self.build_iso_payload(
            &meta_data,
            &user_data,
            network_config.as_deref(),
            &scripts,
            &script_manifest,
        )
    }
}

impl CloudInit {
    fn render_meta_data(&self, machine: &Machine) -> Result<String, Seeders> {
        let mut context = Context::new();
        context.insert("identifier", machine.identifier.value.as_str());
        context.insert("hostname", &machine.name);

        self.render_template("cloud-init/meta-data", &context)
    }

    fn render_user_data(&self, machine: &Machine, has_scripts: bool) -> Result<String, Seeders> {
        let mut context = Context::new();
        let users = machine
            .users
            .value
            .iter()
            .map(|user| {
                let mut item = std::collections::BTreeMap::new();
                item.insert("username", user.username.value.clone());
                item.insert("password", user.password.value.clone());
                item
            })
            .collect::<Vec<_>>();
        let packages = self.packages_context(machine)?;

        context.insert("users", &users);
        context.insert("packages", &packages);
        context.insert("has_scripts", &has_scripts);

        self.render_template("cloud-init/user-data", &context)
    }

    fn render_template(
        &self,
        template: &'static str,
        context: &Context,
    ) -> Result<String, Seeders> {
        Engine::new()
            .and_then(|engine| engine.render(template, context))
            .map_err(|source| Seeders::Template { source })
    }

    fn render_script_context(
        &self,
        machine: &Machine,
        parent: &Path,
    ) -> Result<Vec<std::collections::BTreeMap<&'static str, String>>, Seeders> {
        let Some(scripts) = &machine.scripts else {
            return Ok(Vec::new());
        };
        let mut context = Vec::with_capacity(scripts.len());

        for script in scripts.iter() {
            let path = if script.path.value.is_absolute() {
                script.path.value.clone()
            } else {
                parent.join(&script.path.value)
            };
            let content = self.read_script(&path)?;
            let timeout_seconds = script
                .timeout_seconds
                .as_ref()
                .map_or(300, |timeout| timeout.value);
            let on_failure = script
                .on_failure
                .as_ref()
                .map_or("warn", |on_failure| on_failure.value.as_str());
            let filename = path
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .unwrap_or("script")
                .to_owned();
            let mut item = std::collections::BTreeMap::new();
            item.insert("path", format!("scripts/{}", filename));
            item.insert("timeout_seconds", timeout_seconds.to_string());
            item.insert("on_failure", on_failure.to_owned());
            item.insert("content", content);

            context.push(item);
        }

        Ok(context)
    }

    fn render_script_manifest(
        &self,
        scripts: &[std::collections::BTreeMap<&'static str, String>],
    ) -> String {
        if scripts.is_empty() {
            return String::new();
        }

        let scripts = scripts
            .iter()
            .filter_map(|script| {
                Some(serde_json::json!({
                    "path": script.get("path")?,
                    "timeout_seconds": script.get("timeout_seconds")?.parse::<u32>().ok()?,
                    "on_failure": script.get("on_failure")?,
                }))
            })
            .collect::<Vec<_>>();

        serde_json::to_string_pretty(&serde_json::json!({ "scripts": scripts }))
            .unwrap_or_else(|_| "{\"scripts\":[]}".to_owned())
    }

    fn packages_context(&self, machine: &Machine) -> Result<Option<serde_json::Value>, Seeders> {
        let Some(packages) = &machine.packages else {
            return Ok(None);
        };

        let package_manager = packages.manager.value.into_package_manager();
        let package_names = packages
            .install
            .value
            .iter()
            .map(|package| package.value.as_str())
            .collect::<Vec<_>>();
        let package_install_command = package_manager
            .install_packages_command(&package_names)
            .ok_or_else(|| Seeders::PackageInstallCommandUnavailable {
                manager: package_manager.as_str(),
            })?;
        let mut context = serde_json::Map::new();

        if let Some(command) =
            package_manager.install_package_manager_command(&machine.operating_system)
        {
            context.insert("setup".to_owned(), serde_json::json!(command));
        }

        context.insert(
            "install".to_owned(),
            serde_json::json!(package_install_command.lines().collect::<Vec<_>>()),
        );

        Ok(Some(serde_json::Value::Object(context)))
    }

    fn build_iso_payload(
        &self,
        meta_data: &str,
        user_data: &str,
        network_config: Option<&str>,
        scripts: &[std::collections::BTreeMap<&'static str, String>],
        script_manifest: &str,
    ) -> Result<Vec<u8>, Seeders> {
        let mut files = vec![
            IsoFile::File {
                name: Arc::new("meta-data".to_owned()),
                contents: meta_data.as_bytes().to_vec(),
            },
            IsoFile::File {
                name: Arc::new("user-data".to_owned()),
                contents: user_data.as_bytes().to_vec(),
            },
        ];

        if let Some(network_config) = network_config {
            files.push(IsoFile::File {
                name: Arc::new("network-config".to_owned()),
                contents: network_config.as_bytes().to_vec(),
            });
        }

        if !script_manifest.is_empty() {
            files.push(IsoFile::File {
                name: Arc::new("scripts.json".to_owned()),
                contents: script_manifest.as_bytes().to_vec(),
            });
        }

        for script in scripts {
            if let (Some(path), Some(content)) = (script.get("path"), script.get("content")) {
                files.push(IsoFile::File {
                    name: Arc::new(path.to_owned()),
                    contents: content.as_bytes().to_vec(),
                });
            }
        }

        let input = InputFiles {
            path_separator: PathSeparator::ForwardSlash,
            files,
        };
        let options = options::FormatOptions {
            volume_name: "cidata".to_owned(),
            system_id: None,
            volume_set_id: None,
            publisher_id: None,
            preparer_id: None,
            application_id: None,
            sector_size: 2048,
            path_separator: PathSeparator::ForwardSlash,
            features: options::CreationFeatures::with_extensions(),
            strict_charset: false,
        };
        let estimated_size = estimator::estimate(&input, &options).minimum_bytes() as usize;
        let mut buffer = Cursor::new(vec![0u8; estimated_size]);

        IsoImageWriter::format_new(&mut buffer, input, options).map_err(|error| {
            Seeders::IsoCreationFailed {
                reason: error.to_string(),
            }
        })?;

        // Keep the full allocated buffer: the ISO writer reserves sectors that
        // are not counted in the final write position, and truncating them
        // produces a malformed image.
        Ok(buffer.into_inner())
    }

    fn read_script(&self, path: &Path) -> Result<String, Seeders> {
        let bytes = std::fs::read(path).map_err(|error| Seeders::ScriptReadFailed {
            path: path.display().to_string(),
            reason: error.to_string(),
        })?;

        if bytes.starts_with(&[0xff, 0xfe]) {
            let mut chunks = bytes[2..].chunks_exact(2);

            if !chunks.remainder().is_empty() {
                return Err(Seeders::ScriptReadFailed {
                    path: path.display().to_string(),
                    reason: "UTF-16LE script has an odd byte length".to_owned(),
                });
            }

            let units = chunks
                .by_ref()
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect::<Vec<_>>();

            return String::from_utf16(&units).map_err(|error| Seeders::ScriptReadFailed {
                path: path.display().to_string(),
                reason: error.to_string(),
            });
        }

        let bytes = bytes.strip_prefix(&[0xef, 0xbb, 0xbf]).unwrap_or(&bytes);

        String::from_utf8(bytes.to_vec()).map_err(|error| Seeders::ScriptReadFailed {
            path: path.display().to_string(),
            reason: error.to_string(),
        })
    }
}
