# Usage Guide

This guide covers all MALINA commands and how to use them effectively.

## Command Overview

MALINA provides the following main commands:

| Command | Description |
|---------|-------------|
| `malina deploy` | Deploy a laboratory from configuration file |
| `malina validate` | Validate a configuration file |
| `malina completions` | Generate shell completion scripts |

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

## Workflow Examples

### Quick Start

1. Create a configuration file
2. Validate it: `malina validate lab.toml`
3. Deploy: `malina deploy lab.toml`
4. Access VMs via SSH or VNC (depending on setup)

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
| "Permission denied" | Add user to libvirt group or use sudo |
