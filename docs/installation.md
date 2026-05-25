# Installation

This guide covers different ways to install MALINA on your system.

## Prerequisites 

Before installing MALINA, ensure you have the following:

- **Rust toolchain** (Cargo) - [Install Rust](https://www.rust-lang.org/tools/install)
- **KVM/QEMU** for virtualization support
  - Linux: `sudo apt install qemu-kvm libvirt-daemon-system libvirt-clients bridge-utils libvirt-devel`

## Installation Methods

### Method 1: Cargo (Recommended)

The easiest way to install MALINA is using Cargo from crates.io:

```sh
cargo install malina
```

This will download and compile the latest stable release.

### Method 2: From Source

To build from source, clone the repository and compile:

```sh
git clone https://github.com/AntwortEinesLebens/MALINA.git
cd MALINA
cargo install --path .
```

## Shell Completion

After installation, enable shell completion for your preferred shell:

| Shell    | Command                                    |
| -------- | ------------------------------------------ |
| Bash     | `malina completions bash >> ~/.bashrc`     |
| Elvish   | `malina completions elvish >> ~/.config/elvish/rc.elv` |
| Fish     | `malina completions fish > ~/.config/fish/completions/malina.fish` |
| PowerShell | `malina completions powershell >> $PROFILE` |
| Zsh      | `malina completions zsh >> ~/.zshrc`       |

### Powershell completion

For powershell completion, you need to install libvirt and libvirt-devel [crates.io](https://crates.io/crates/virt)

## Verification

Verify the installation by checking the version:

```sh
malina --version
```

## Uninstallation

To remove MALINA from your system:

### Cargo Installation

```sh
cargo uninstall malina
```

### Manual Removal
Linux
```sh
sudo rm /usr/local/bin/malina
# Remove configuration files if needed
rm -rf ~/.config/malina
```

Windows  
Remove the project files

## Updating

When a new version is available, update using:

```sh
cargo install --locked malina
```

Or check for updates on [GitHub Releases](https://github.com/AntwortEinesLebens/MALINA/releases).