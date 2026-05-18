<!--
SPDX-FileCopyrightText: 2025 The MALINA development team

SPDX-License-Identifier: CC0-1.0
-->

<div align="center">
  <a href="https://github.com/AntwortEinesLebens/MALINA">
    <!-- markdownlint-disable-next-line line-length -->
    <img src="https://raw.githubusercontent.com/AntwortEinesLebens/MALINA/refs/heads/main/assets/images/logo.svg" alt="MALINA Logo"/>
  </a>

<h1 align="center">MALINA</h1>

<p align="center">
    <!-- markdownlint-disable-next-line line-length -->
    Modular Automated Laboratories for Investigating Nefarious Artifacts<br />
    Deploy and manage stealthy, reproducible malware analysis laboratories from configuration files
    <br /><br />
    <a href="https://github.com/AntwortEinesLebens/MALINA/issues/">
      Report Bug
    </a>
    ·
    <a href="https://github.com/AntwortEinesLebens/MALINA/issues/">
      Request Feature
    </a>
    <br /><br />
    <a href="https://github.com/AntwortEinesLebens/MALINA">
      <!-- markdownlint-disable-next-line line-length -->
      <img src="https://img.shields.io/badge/GitHub-181717?logo=github&logoColor=fff&style=for-the-badge" alt="GitHub badge" />
    </a>
    <a href="./LICENSES/GPL-3.0-or-later.txt">
      <!-- markdownlint-disable-next-line line-length -->
      <img src="https://img.shields.io/badge/License-GPL%203.0%20or%20later-green.svg?style=for-the-badge" alt="GPL 3.0 or later badge" />
    </a>
    <a href="https://www.rust-lang.org/">
      <!-- markdownlint-disable-next-line line-length -->
      <img src="https://img.shields.io/badge/Rust-000?logo=rust&logoColor=fff&style=for-the-badge" alt="Rust badge" />
    </a>
    <a href="https://reuse.software/">
      <!-- markdownlint-disable-next-line line-length -->
      <img src="https://img.shields.io/reuse/compliance/github.com%2FAntwortEinesLebens%2FMALINA?style=for-the-badge" alt="REUSE badge" />
    </a>
  </p>
</div>

## 📋 Quick Start

```sh
# Install MALINA
cargo install malina

# Deploy a laboratory from configuration
malina deploy lab-config.toml

# Manage your laboratories
malina laboratories list
malina laboratories start my-lab
malina laboratories stop my-lab
```

## ⚠️ Disclaimer

This project is currently under heavy development. Features may be incomplete, unstable, or subject to change without notice. It is not recommended for production use at this time. Use at your own risk and expect breaking changes as the project evolves.

At this stage, deployed environments are also likely to be flagged as virtual machines because stealth hardening is not implemented yet. The current focus is development, functionality, and architecture iteration rather than anti-detection guarantees.

## 🎯 What is MALINA?

**Modular Automated Laboratories for Investigating Nefarious Artifacts** ([MALINA]) is a tool designed to help security researchers and malware analysts create reproducible, stealthy, and unique analysis environments. By leveraging a simple configuration file, [MALINA] automates the deployment of virtual machines pre-configured with analysis tools, anti-detection patches, and realistic system activity.

Each laboratory built with [MALINA] is both **reproducible** and **unique**:  
- **Reproducible**: The same configuration yields the same toolset and patches  
- **Unique**: Each deployment generates a distinct system footprint to prevent fingerprinting

Think of it as infrastructure as code tailored specifically for malware analysis workflows.

## ✨ Key Features

- 🚀 **Easy Deployment** - Deploy labs from TOML configuration files
- 🔒 **Network Isolation** - Support for isolated, private, and public network modes
- 💻 **Multi-OS Support** - Linux and Windows virtual machines
- 🛠️ **Tool Pre-installation** - Configure packages and scripts per VM
- 🎯 **Anti-Detection Ready** - Framework for stealth hardening (in development)
- 📊 **Reproducible Environments** - Version-controlled lab configurations
- 🔍 **Diagnostics & Validation** - Built-in tools to verify setups

## 📜 License

Distributed under the [GPL 3.0 or later] license.

## 🔗 Links

- [GitHub Repository](https://github.com/AntwortEinesLebens/MALINA)
- [Issue Tracker](https://github.com/AntwortEinesLebens/MALINA/issues)
- [REUSE Compliance](https://reuse.software/)

---

<p align="center">
  Made with ❤️ by the MALINA development team
</p>

[cargo]: https://doc.rust-lang.org/stable/cargo/
[crates.io]: https://crates.io/
[gpl 3.0 or later]: https://github.com/AntwortEinesLebens/MALINA/LICENSES/GPL-3.0-or-later.txt
[malina]: https://github.com/AntwortEinesLebens/MALINA/