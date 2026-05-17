<!--
SPDX-FileCopyrightText: 2025 The MALINA development team

SPDX-License-Identifier: CC0-1.0
-->

# Changelog

All notable changes to MALINA will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of MALINA v0.1.0
- Basic laboratory deployment functionality
- KVM/QEMU virtualization provider support
- Configuration file-based lab definitions
- Laboratory management commands (list, start, stop, destroy)
- System diagnostics and validation tools

### Changed
- None yet

### Deprecated
- None yet

### Removed
- None yet

### Fixed
- Initial bug fixes and stability improvements

## [0.1.0] - YYYY-MM-DD

### Added
- **Core functionality**: Deploy malware analysis laboratories from TOML configuration files
- **Virtualization support**: KVM/QEMU provider for Linux virtual machines
- **Multi-machine support**: Define multiple VMs in a single configuration file
- **Linux and Windows support**: Support for both Linux distributions and Windows operating systems
- **Package management**: Automated package installation on Linux VMs (apt, dnf, pacman)
- **Script execution**: PowerShell script execution on Windows VMs
- **User management**: Create and configure users in virtual machines
- **Network isolation**: Support for isolated, private, and public network modes
- **Validation tools**: Configuration validation before deployment
- **System diagnostics**: `malina doctor` command to check system readiness
- **Deployment diagnosis**: `malina diagnose` command to troubleshoot failed deployments
- **Shell completion**: Completion scripts for Bash, Zsh, Fish, Elvish, and PowerShell

### Changed
- Project structure organized around modular commands
- Error handling improved with miette library for better user feedback

### Fixed
- Initial stability improvements
- Configuration parsing edge cases

## [0.0.x] - Previous versions

See the [GitHub releases page](https://github.com/AntwortEinesLebens/MALINA/releases) for detailed changelog entries of previous pre-release versions.

---

## Version History Guidelines

When contributing changes, please follow these guidelines:

1. **Use conventional commits**: Prefix your commit messages with `feat:`, `fix:`, `chore:`, etc.
2. **Update this file**: Add your changes under the appropriate section
3. **Be specific**: Describe what changed and why it matters to users
4. **Tag releases**: Use semantic versioning when creating releases

## Contributing to Changelog

If you want to contribute to the changelog:

1. Create a pull request with your feature or fix
2. Include a description of changes in the PR
3. The maintainers will update this file during release preparation

---

**Note**: This is an early-stage project under active development. Breaking changes may occur without notice until v1.0.0.
