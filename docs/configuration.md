# Configuration

MALINA uses TOML configuration files to define laboratory setups. This guide explains the available options and how to create your own configurations.

## Basic Structure

A MALINA configuration file consists of:

- **`version`**: The configuration format version (currently `1`)
- **`[laboratory]`**: General laboratory settings
- **`[[machines]]`**: One or more virtual machine definitions

## Laboratory Section

The `[laboratory]` section defines global properties for the entire laboratory:

```toml
version = 1

[laboratory]
name = "my-lab"           # Unique identifier for this lab
network = "isolated"      # Network isolation mode (see below)
provider = "kvm"          # Virtualization provider (currently only KVM supported)
```

### Network Modes

MALINA supports different network isolation levels:

- **`isolated`**: VMs have no external network access (default for security)
- **`private`**: VMs can communicate with each other but not externally
- **`public`**: Full network connectivity (not recommended for malware analysis)

## Machine Configuration

Each `[[machines]]` block defines a virtual machine. You can define multiple machines in one configuration file.

### Hardware Resources

```toml
[[machines]]
identifier = "analysis-vm"  # Unique identifier within this config
name = "Analysis VM"        # Display name

[machines.hardware]
cpus = 2                    # Number of CPU cores (default: 1)
memory_megabyte = 4096      # Memory in MB (default: 512)
```

### Operating System

Define the guest operating system:

```toml
[machines.operating_system]
family = "linux"           # or "windows"
distribution = "debian"    # for Linux, e.g., "ubuntu", "fedora"
version = "13"             # OS version (e.g., "12", "13", "14")

# For Windows:
family = "windows"
version = "11"             # e.g., "10", "11"
```

### Disk Image

Specify the disk image to use:

```toml
image = "./images/debian-13.qcow2"  # Path to disk image
# or for Windows with cloudbase-init:
image = "./images/windows-11.qcow2"
```

**Note**: For Windows images, ensure they are pre-configured with `cloudbase-init` for proper MALINA integration.

### Users

Add users to the virtual machine:

```toml
[[machines.users]]
username = "analyst"
password = "secure_password_here"  # Use strong passwords in production
```

Multiple users can be defined by adding additional `[[machines.users]]` blocks.

### Packages (Linux)

Install packages on Linux VMs:

```toml
[machines.packages]
manager = "apt"           # or "dnf", "pacman", etc.
install = [               # List of package names
    "ghidra",
    "radare2",
    "strings",
    "file"
]
```

### Scripts (Windows)

Execute PowerShell scripts on Windows VMs:

```toml
[[machines.scripts]]
path = "./scripts/disable-defender.ps1"
timeout_seconds = 800     # Maximum execution time in seconds
on_failure = "warn"       # or "error", "ignore"
```

Multiple scripts can be defined by adding additional `[[machines.scripts]]` blocks.

## Complete Example

Here's a complete example configuration:

```toml
version = 1

[laboratory]
name = "malware-analysis-lab"
network = "isolated"
provider = "kvm"


[[machines]]
identifier = "linux-host"
name = "Linux Analysis Host"

[machines.hardware]
cpus = 4
memory_megabyte = 8192

[machines.operating_system]
family = "linux"
distribution = "ubuntu"
version = "24.04"
image = "./images/ubuntu-24.04.qcow2"

[[machines.users]]
username = "analyst"
password = "secure_password_123"


[[machines]]
identifier = "windows-host"
name = "Windows Analysis Host"

[machines.hardware]
cpus = 4
memory_megabyte = 8192

[machines.operating_system]
family = "windows"
version = "11"
image = "./images/windows-11.qcow2"

[[machines.users]]
username = "analyst"
password = "secure_password_123"

[machines.packages]
manager = "winget"
install = ["Microsoft.WinDbg"]

[[machines.scripts]]
path = "./scripts/disable-defender.ps1"
timeout_seconds = 800
on_failure = "warn"
```

## Configuration Validation

Before deploying a laboratory, validate your configuration:

```sh
malina validate lab-config.toml
```

This checks for syntax errors and ensures all required fields are present.

## Best Practices

1. **Use strong passwords**: Never use default or weak passwords in production configurations
2. **Version control**: Store configuration files in version control (excluding disk images)
3. **Modular designs**: Split large configurations into multiple files when needed
4. **Testing**: Always validate configurations before deployment
5. **Documentation**: Comment complex configurations for future reference

## Template Examples

Check the `examples/` directory for pre-configured templates:

- `double-virtual-machines.toml`: A two-VM analysis environment with Linux and Windows hosts

For more examples, visit the [GitHub repository](https://github.com/AntwortEinesLebens/MALINA/tree/main/examples).