# Troubleshooting

This guide helps you resolve common issues with MALINA.

## Quick Reference

| Issue | Command to Run |
|-------|---------------|
| Check system readiness | `malina doctor` |
| Validate configuration | `malina validate config.toml` |
| Diagnose deployment issues | `malina diagnose lab-name` |
| View verbose errors | `malina -v deploy config.toml` |

## Installation Issues

### "cargo install" fails with network error

**Problem**: Unable to download MALINA from crates.io.

**Solution**:
```sh
# Check internet connection
ping crates.io

# Try alternative mirror
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
cargo install malina

# Or build from source
git clone https://github.com/AntwortEinesLebens/MALINA.git
cd MALINA
cargo install --path .
```

### Rust version too old

**Problem**: "rustc 1.88.0 or later required" error.

**Solution**:
```sh
# Check current version
rustup show

# Update Rust toolchain
rustup update
rustup default stable

# Verify installation
cargo --version
```

### Permission denied errors

**Problem**: Cannot write to configuration directory.

**Solution**:
```sh
# Create config directory with proper permissions
mkdir -p ~/.config/malina
chmod 755 ~/.config/malina

# Or use sudo (not recommended for regular operations)
sudo malina deploy config.toml
```

## KVM/QEMU Issues

### "KVM not available" error

**Problem**: Virtualization modules not loaded or not accessible.

**Solution**:
```sh
# Check if KVM is supported
kvm-ok

# Load kernel modules (Linux)
sudo modprobe kvm
sudo modprobe kvm-intel  # or kvm-amd for AMD

# Verify modules are loaded
lsmod | grep kvm

# Restart libvirt daemon
sudo systemctl restart libvirtd
```

### "libvirt connection failed"

**Problem**: Cannot connect to libvirt daemon.

**Solution**:
```sh
# Check if libvirt is running
systemctl status libvirtd

# Start libvirt service
sudo systemctl start libvirtd

# Check user permissions
groups | grep libvirt

# Add user to libvirt group (if needed)
sudo usermod -aG libvirt $USER
newgrp libvirt  # Log out and back in for changes to take effect
```

### QEMU binary not found

**Problem**: MALINA cannot locate QEMU executable.

**Solution**:
```sh
# Check QEMU installation
which qemu-system-x86_64

# Install QEMU if missing (Debian/Ubuntu)
sudo apt install qemu-kvm libvirt-daemon-system

# Or check alternative paths
find /usr -name "qemu-system-*" 2>/dev/null
```

## Configuration Issues

### TOML syntax errors

**Problem**: "Invalid TOML" or parsing errors.

**Solution**:
```sh
# Validate configuration first
malina validate config.toml

# Use a TOML validator online or with:
python3 -c "import tomllib; tomllib.load(open('config.toml', 'rb'))"  # Python 3.11+
```

### Missing required fields

**Problem**: Configuration validation fails due to missing fields.

**Solution**: Ensure all required sections are present:

```toml
version = 1

[laboratory]
name = "your-lab-name"      # Required
network = "isolated"         # Required
provider = "kvm"             # Required

[[machines]]
identifier = "unique-id"     # Required
# ... other required fields based on your setup
```

### Invalid disk image path

**Problem**: Cannot find specified disk image.

**Solution**:
```sh
# Check if image exists
ls -la ./images/your-image.qcow2

# Use absolute paths or ensure relative paths are correct
image = "/full/path/to/image.qcow2"
```

### Image format errors

**Problem**: Disk image is not in the expected format.

**Solution**:
```sh
# Check QEMU image format
qemu-img info ./images/your-image.qcow2

# Convert if necessary (example: raw to qcow2)
qemu-img convert -f raw -O qcow2 source.raw dest.qcow2
```

## Deployment Issues

### VM fails to start

**Problem**: Virtual machine cannot boot.

**Solution**:
```sh
# Diagnose the specific lab
malina diagnose your-lab-name

# Check logs in MALINA config directory
cat ~/.config/malina/logs/*.log

# Try starting manually with libvirt CLI
virsh list --all
virsh start your-vm-name
```

### Resource allocation errors

**Problem**: Insufficient CPU or memory resources.

**Solution**:
```sh
# Check available resources
free -h                    # Memory
nproc                       # CPUs
df -h /                     # Disk space

# Adjust configuration accordingly
# Reduce cpus and/or memory_megabyte in your config.toml
```

### Network isolation issues

**Problem**: VMs cannot communicate or have unexpected network access.

**Solution**:
```sh
# Check libvirt network status
virsh net-list --all

# Verify network mode matches configuration
grep "network" config.toml

# For isolated networks, ensure no external bridges are configured
virsh net-dhcp-leases default  # Check for unexpected leases
```

## Runtime Issues

### VM crashes or freezes

**Problem**: Virtual machine becomes unresponsive.

**Solution**:
```sh
# Force stop the lab (graceful shutdown may not work)
malina laboratories destroy your-lab-name

# Or use libvirt directly
virsh destroy your-vm-name
virsh undefine your-vm-name --remove-all-storage
```

### Slow performance

**Problem**: Laboratory runs slower than expected.

**Solution**:
1. Check for competing processes: `top` or `htop`
2. Verify CPU pinning is not causing issues
3. Ensure disk I/O is not bottlenecked (use SSD if possible)
4. Reduce VM resource allocation in configuration

### Package installation failures

**Problem**: Configured packages fail to install.

**Solution**:
```sh
# Check package manager logs
journalctl -xe | grep apt  # or dnf, pacman depending on distro

# Verify package availability
apt search <package-name>   # or equivalent for your distro

# Update package lists before installation
sudo apt update
```

## Diagnostic Commands

### Get detailed system information

```sh
# Full system check
malina doctor -v

# Check specific components
dmesg | grep kvm           # Kernel messages about KVM
systemctl status libvirtd  # Libvirt service status
qemu-system-x86_64 --version  # QEMU version
```

### View MALINA logs

```sh
# Find log directory
ls -la ~/.config/malina/logs/

# View latest log
tail -f ~/.config/malina/logs/latest.log

# Search for errors
grep -i error ~/.config/malina/logs/*.log
```

## Common Error Messages

### "Failed to connect to libvirt"

**Cause**: Libvirt daemon not running or connection refused.

**Fix**: `sudo systemctl start libvirtd` and verify with `systemctl status libvirtd`

### "Cannot allocate memory"

**Cause**: System out of RAM or swap space exhausted.

**Fix**: Close other applications, add swap space, or reduce VM memory allocation

### "Permission denied: /dev/kvm"

**Cause**: User lacks access to KVM device.

**Fix**: Add user to libvirt group: `sudo usermod -aG libvirt $USER`

### "Disk image is locked"

**Cause**: Another process has the disk image open.

**Fix**: Find and close the process, or use a different image path

## Getting More Help

If you've tried these solutions and still have issues:

1. **Run diagnostics**: `malina doctor` and `malina diagnose lab-name`
2. **Check logs**: Review files in `~/.config/malina/logs/`
3. **Search existing issues**: [GitHub Issues](https://github.com/AntwortEinesLebens/MALINA/issues)
4. **Create a new issue**: Include output of diagnostic commands and error messages

## Reporting Bugs

When reporting bugs, include:

- MALINA version (`malina --version`)
- Operating system and version
- Output of `malina doctor`
- Relevant configuration snippets (remove sensitive data)
- Error messages and stack traces
- Steps to reproduce the issue

Thank you for helping improve MALINA!