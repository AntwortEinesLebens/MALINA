# Contributing to MALINA

Thank you for your interest in contributing to MALINA! This guide will help you get started with contributing to the project.

## Code of Conduct

We want MALINA to be an inclusive community. Please adhere to these principles:

- Be respectful and considerate
- Focus on constructive feedback
- Assume good intentions
- Welcome newcomers

## How to Contribute

There are many ways to contribute, whether you're a seasoned developer or just starting out.

### Reporting Issues

Found a bug or have a feature request? Please [open an issue](https://github.com/AntwortEinesLebens/MALINA/issues) with:

1. **Clear title and description**
2. **Steps to reproduce** (for bugs)
3. **Expected vs actual behavior**
4. **Environment details**: OS, MALINA version, etc.
5. **Logs or error messages** if applicable

### Feature Requests

When suggesting new features:

- Explain the use case and problem it solves
- Provide examples of desired functionality
- Consider security implications (especially for malware analysis tools)
- Check if similar features already exist in issues

## Development Setup

### Prerequisites

- Rust 1.88.0 or later
- Git
- Code editor (VS Code, Vim, etc.)

### Clone and Build

```sh
# Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/MALINA.git
cd MALINA

# Install dependencies
cargo build

# Run tests
cargo test

# Format code
cargo fmt
```

### Development Workflow

1. **Create a branch** for your feature or bugfix:
   ```sh
   git checkout -b feature/your-feature-name
   ```

2. **Make changes** and follow Rust best practices

3. **Write tests** for new functionality

4. **Update documentation** as needed

5. **Run the full test suite**:
   ```sh
   cargo test --all-features
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

6. **Submit a pull request** to the main repository

## Code Style

MALINA follows Rust community standards:

### Formatting

Use `cargo fmt` for consistent code formatting. The project uses `rustfmt.toml` configuration.

### Linting

Run `cargo clippy` to catch common issues and style violations. All warnings should be addressed before submitting PRs.

### Documentation

- Add doc comments to public functions and structs
- Update README.md for user-facing changes
- Keep inline comments concise and explanatory

Example:
```rust
/// Validates the laboratory configuration file path.
/// Returns an error if the file doesn't exist or is invalid TOML.
pub fn validate_config(path: PathBuf) -> Result<()> {
    // ... implementation
}
```

## Testing

### Unit Tests

Write tests for individual functions and modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_config() {
        let valid_path = PathBuf::from("valid.toml");
        assert!(validate_config(valid_path).is_ok());
    }
}
```

### Integration Tests

Test end-to-end functionality:

```sh
cargo test --tests
```

### Test Coverage

Maintain reasonable test coverage. Check current coverage with:

```sh
cargo tarpaulin --out Html
```

## Documentation

Documentation is crucial for user adoption. When making changes:

1. **Update relevant docs** in the `docs/` directory
2. **Add examples** where helpful
3. **Keep it clear and concise**
4. **Use code blocks** for commands and configurations

### MkDocs Structure

The documentation uses [MkDocs](https://www.mkdocs.org/):

```
mkdocs.yml          # Configuration file
docs/
  index.md          # Homepage
  installation.md   # Installation guide
  configuration.md  # Configuration reference
  usage.md          # Usage examples
  contributing.md   # This file
```

## Security Considerations

Since MALINA is a malware analysis tool, security is paramount:

- **Review all code** for potential vulnerabilities
- **Test thoroughly** before merging
- **Consider attack vectors** in new features
- **Follow secure coding practices**

## Pull Request Process

1. **Fork the repository**
2. **Create a branch**: `git checkout -b feature/amazing-feature`
3. **Commit changes**: `git commit -m 'Add amazing feature'`
4. **Push to fork**: `git push origin feature/amazing-feature`
5. **Open a pull request** with:
   - Clear title and description
   - Reference related issues
   - List of changes made
   - Screenshots if UI-related

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] All tests pass
- [ ] Documentation updated
- [ ] No new warnings from clippy
- [ ] Changes are backward compatible (or migration guide provided)
- [ ] Security implications considered

## Review Process

All contributions go through review:

1. **Maintainers review** your PR for quality and correctness
2. **Feedback is provided** with constructive suggestions
3. **Iterate on changes** as needed
4. **Merge upon approval**

### Typical Review Time

- Minor fixes: 1-3 days
- Features: 3-7 days
- Major changes: 1-2 weeks

## Areas Needing Attention

These areas are particularly welcome for contributions:

- [ ] More configuration examples
- [ ] Additional virtualization providers (beyond KVM)
- [ ] Enhanced anti-detection features
- [ ] Better error messages and diagnostics
- [ ] Windows image automation scripts
- [ ] Documentation improvements
- [ ] Test coverage expansion

## Getting Help

Need assistance? Join the community:

- **GitHub Issues**: Ask questions in existing issues or start a new one
- **Discussions**: Share ideas and get feedback

## License

By contributing to MALINA, you agree that your contributions will be licensed under the [GPL 3.0 or later](LICENSES/GPL-3.0-or-later.txt) license.

## Acknowledgments

Thank you for helping make MALINA better! Your contributions support the malware analysis community and improve security research capabilities worldwide.