# Usage Guide

This guide covers all MALINA commands and how to use them effectively.

## Command Overview

MALINA provides the following main commands:

| Command | Description |
|---------|-------------|
| `malina deploy` | Deploy a laboratory from configuration file |
| `malina validate` | Validate a configuration file |
| `malina doctor` | Check system readiness for deployment |
| `malina diagnose` | Diagnose failed or partial deployments |
| `malina completions` | Generate shell completion scripts |
| `malina laboratories` | Manage deployed laboratories |

## Deploying a Laboratory

Deploy a laboratory from a configuration file:

```sh
malina deploy lab-config.toml
```

This will:

1. Validate the configuration
2. Create necessary directories and files
3. Start virtual machines according to the configuration
4. Apply anti-detection patches (when available)
5. Install configured packages and run scripts

### Deployment Options

You can use flags with the deploy command:

```sh
malina deploy -v lab-config.toml    # Verbose output
malina deploy --quiet lab-config.toml  # Suppress non-essential messages
```

## Managing Laboratories

After deployment, you can manage your laboratories using the `laboratories` subcommand.

### List All Laboratories

View all deployed laboratories:

```sh
malina laboratories list
```

Output example:
```
NAME                    STATUS      PROVIDER    CREATED
double-virtual-machines running     kvm         2025-01-15 10:30
security-lab            stopped     kvm         2025-01-14 08:15
```

### Start a Laboratory

Start a stopped laboratory:

```sh
malina laboratories start lab-name
```

### Stop a Laboratory

Stop a running laboratory gracefully:

```sh
malina laboratories stop lab-name
```

This shuts down VMs properly to preserve state.

### Destroy a Laboratory

Completely remove a laboratory and all its resources:

```sh
malina laboratories destroy lab-name
```

**Warning**: This action cannot be undone. All data in the laboratory will be lost.

## Validating Configuration

Before deploying, always validate your configuration file:

```sh
malina validate config.toml
```

This checks for:

- TOML syntax errors
- Missing required fields
- Invalid values
- File accessibility issues

Example output:
```
✓ Configuration is valid
  - Laboratory name: my-lab
  - Provider: kvm
  - Machines: 2
    ✓ linux-analysis (4 CPUs, 8GB RAM)
    ✓ windows-analysis (4 CPUs, 8GB RAM)
```

## System Diagnostics

### Doctor Command

Check if your system is ready for laboratory deployment:

```sh
malina doctor
```

This verifies:

- KVM/QEMU installation and availability
- Required kernel modules loaded
- Sufficient disk space
- Network configuration
- User permissions

Example output:
```
System Readiness Check
======================

✓ KVM is installed and accessible
✓ Kernel modules loaded (kvm, kvm-intel)
✓ Disk space available: 500GB free
✓ libvirt daemon running
✓ User has required permissions

All checks passed! Your system is ready for deployment.
```

### Diagnose Command

Diagnose issues with a failed or partially deployed laboratory:

```sh
malina diagnose lab-name
```

This helps identify problems such as:

- VM startup failures
- Configuration errors
- Resource conflicts
- Permission issues

## Shell Completion

Generate completion scripts for your shell to improve command-line experience:

```sh
# For Bash
malina completions bash >> ~/.bashrc
source ~/.bashrc

# For Zsh
malina completions zsh >> ~/.zshrc

# For Fish
malina completions fish > ~/.config/fish/completions/malina.fish
```

## Workflow Examples

### Quick Start

1. Create a configuration file
2. Validate it: `malina validate lab.toml`
3. Deploy: `malina deploy lab.toml`
4. Access VMs via SSH or VNC (depending on setup)

### Daily Analysis Workflow

```sh
# Check system status
malina doctor

# List available labs
malina laboratories list

# Start analysis environment
malina laboratories start malware-lab

# When done, stop the lab
malina laboratories stop malware-lab
```

### Creating Multiple Environments

Create separate configuration files for different scenarios:

- `basic-analysis.toml`: Single Linux VM with common tools
- `advanced-analysis.toml`: Multi-VM environment with Windows host
- `forensics-lab.toml`: Specialized forensics tools setup

## Tips and Tricks

1. **Use descriptive names**: Choose meaningful laboratory names for easy identification
2. **Version your configs**: Include version information in config comments
3. **Test configurations**: Always validate before deploying to production environments
4. **Backup images**: Keep disk image backups outside the MALINA configuration directory
5. **Document scripts**: Comment PowerShell and shell scripts used in configurations

## Error Handling

MALINA provides detailed error messages when issues occur:

```sh
malina deploy broken-config.toml
# Output includes specific error location and suggestions
```

Common errors and solutions:

| Error | Solution |
|-------|----------|
| "Configuration file not found" | Check the path is correct |
| "Invalid TOML syntax" | Validate with `malina validate` first |
| "KVM not available" | Run `malina doctor` to check system requirements |
| "Permission denied" | Add user to libvirt group or use sudo |

## Next Steps

- [Configuration](configuration.md): Learn about configuration options
- [Installation](installation.md): Troubleshoot installation issues
- [Contributing](contributing.md): Help improve MALINA