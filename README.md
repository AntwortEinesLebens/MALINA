<!--
SPDX-FileCopyrightText: 2025 The MALINA development team

SPDX-License-Identifier: GPL-3.0-or-later
-->

<div align="center">
  <a href="https://github.com/AntwortEinesLebens/MALINA">
    <!-- markdownlint-disable-next-line line-length -->
    <img src="https://raw.githubusercontent.com/AntwortEinesLebens/MALINA/refs/heads/main/assets/images/logo.svg" alt="Logo"/>
  </a>

<h3 align="center">MALINA</h3>

<p align="center">
    <!-- markdownlint-disable-next-line line-length -->
    Deploy and manage stealthy, reproducible malware analysis laboratories from configuration files
    <br />
    <a href="https://github.com/AntwortEinesLebens/MALINA/issues/">
      Report Bug
    </a>
    ·
    <a href="https://github.com/AntwortEinesLebens/MALINA/issues/">
      Request Feature
    </a>
    <br />
    <br />
    <a href="https://github.com/AntwortEinesLebens/MALINA">
      <!-- markdownlint-disable-next-line line-length -->
      <img src="https://img.shields.io/badge/GitHub-181717?logo=github&logoColor=fff&style=for-the-badge" alt="Github badge" />
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
      <img src="https://img.shields.io/reuse/compliance/github.com%2FAntwortEinesLebens%2FMALINA?style=for-the-badge" alt="Reuse badge" />
    </a>
  </p>
</div>

## 📋 Table of content

- [📋 Table of content](#-table-of-content)
- [⚠️ Disclaimer](#%EF%B8%8F-disclaimer)
- [👀 About the project](#-about-the-project)
  - [❓ Why](#-why)
- [🚀 Getting started](#-getting-started)
  - [⚙️ Prerequisites](#%EF%B8%8F-prerequisites)
  - [📦 Installation](#-installation)
  - [🥷 Quick examples](#-quick-examples)
- [👷 Contributing](#-contributing)
- [📚 Licenses](#-licenses)

## ⚠️ Disclaimer

At this stage, this is closer to a proof of concept, and the code is still rough
because of the [commit from hell] and AI-generated changes. It will take us some
time to refactor it completely, but we are happy to say that it works in
practice, not just in theory.

This project is currently under heavy development. Features may be incomplete,
unstable, or subject to change without notice. It is not recommended for
production use at this time. Use at your own risk and expect breaking changes
as the project evolves.

At this stage, deployed environments are also likely to be flagged as virtual
machines because stealth hardening is not implemented yet. The current focus is
development, functionality, and architecture iteration rather than anti-
detection guarantees.

## 👀 About the project

**Modular Automated Laboratories for Investigating Nefarious Artifacts** ([MALINA])
is a tool designed to help security researchers and malware analysts create
reproducible, stealthy, and unique analysis environments. By leveraging a
simple configuration file, [MALINA] automates the deployment of virtual machines
pre-configured with analysis tools, anti-detection patches, and realistic
system activity.

Each laboratory built with [MALINA] is both reproducible and unique.
Reproducible means that the same configuration yields the same toolset and
patches. Unique means that each deployment generates a distinct system footprint
to prevent fingerprinting. Think of it as infrastructure as code tailored
specifically for malware analysis workflows.

### ❓ Why

Setting up a malware analysis lab is a time-consuming, error-prone process.
Making it resistant to detection adds another layer of complexity. While
automation scripts can help, they're often custom-made, difficult to share,
and hard to maintain. [MALINA] aims to make malware analysis labs more
accessible, reproducible, and disposable, so you can focus on analyzing
malware, not building environments.

## 🚀 Getting started

This is one way you can install the project yourself.

### ⚙️ Prerequisites

You'll need [Cargo] to get through the installation process.

### 📦 Installation

This will install using [crates.io]. If you need more installation options,
check out this [page][installation section]. To install it, just enter this
command in your preferred terminal:

```sh
cargo install malina
```

To enable shell autocompletion, run the appropriate command for your shell and
add it to your shell's configuration file:

<!-- markdownlint-disable line-length -->

| Shell | Command |
| ---------- | ------------------------------------------------------------------ |
| Bash | `malina completions bash >> ~/.bashrc` |
| Elvish | `malina completions elvish >> ~/.config/elvish/rc.elv` |
| Fish | `malina completions fish > ~/.config/fish/completions/malina.fish` |
| PowerShell | `malina completions powershell >> $PROFILE` |
| Zsh | `malina completions zsh >> ~/.zshrc` |

<!-- markdownlint-enable line-length -->

### 🥷 Quick examples

Now that [MALINA] is installed, you can start deploying your own laboratory! For
example, you can deploy your very first laboratory by doing the following:

```sh
malina deploy laboratory.toml
```

Once deployed, you can manage your laboratories:

```sh
malina laboratories list
malina laboratories start double-virtual-machines
malina laboratories stop double-virtual-machines
malina laboratories destroy double-virtual-machines
```

If you want to know more, check out the [documentation][installation section].

## 👷 Contributing

The open source community is an awesome place to learn, inspire, and create, and
we're grateful for any contributions you can make. If you're interested, we'd
love your help with any kind of work.

## 📚 Licenses

Distributed under the [GPL 3.0 or later] license.

[cargo]: https://doc.rust-lang.org/stable/cargo/
[commit from hell]: https://github.com/AntwortEinesLebens/MALINA/commit/219d98d48142c895b669694ba6ba2ffa8c70ba6f
[crates.io]: https://crates.io/
[gpl 3.0 or later]: ./LICENSES/GPL-3.0-or-later.txt
[installation section]: https://AntwortEinesLebens.github.io/MALINA/installation/
[malina]: https://github.com/AntwortEinesLebens/MALINA/
