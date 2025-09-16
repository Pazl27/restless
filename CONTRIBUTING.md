# Contributing to Restless

Thank you for your interest in contributing to Restless! We welcome contributions from everyone and are grateful for every pull request, bug report, and feature suggestion.

## 🚀 Getting Started

### Development Environment

1. **Install Rust**: Visit [rustup.rs](https://rustup.rs/) to install Rust
2. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/restless.git
   cd restless
   ```
3. **Build and run**:
   ```bash
   cargo run
   ```

### Project Structure

Restless follows a modular architecture:

- `src/app/` - Application state and tab management
- `src/handlers/` - Event handling and user input processing
- `src/logic/` - Core HTTP request/response logic
- `src/ui/` - Terminal user interface components
- `src/terminal/` - Terminal setup and management
- `src/error.rs` - Centralized error handling

## 🛠️ Development Workflow

### Before You Start

1. Check existing [issues](https://github.com/yourusername/restless/issues) and [pull requests](https://github.com/yourusername/restless/pulls)
2. For major features, please open an issue first to discuss the approach
3. Fork the repository and create a feature branch

### Making Changes

1. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our coding standards (see below)

3. **Test your changes**:
   ```bash
   # Run all tests
   cargo test
   
   # Run tests with network access (optional)
   cargo test -- --ignored
   
   # Check formatting
   cargo fmt --check
   
   # Run linter
   cargo clippy -- -D warnings
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: add amazing new feature"
   ```

5. **Push and create a pull request**:
   ```bash
   git push origin feature/your-feature-name
   ```

## 📝 Code Standards

### Rust Guidelines

- **Formatting**: Use `cargo fmt` for consistent formatting
- **Linting**: Fix all `cargo clippy` warnings
- **Documentation**: Add doc comments for public APIs
- **Error Handling**: Use the project's error types (`RestlessError`, etc.)
- **Testing**: Write tests for new functionality

### Code Style

```rust
// ✅ Good: Clear function names and documentation
/// Handles sending an HTTP request for the current tab
pub async fn send_current_request(app: &mut App) -> Result<Option<String>> {
    // Validate request before sending
    if let Err(e) = app.validate_current_request() {
        return Ok(Some(format!("Validation error: {}", e)));
    }
    
    // ... implementation
}

// ❌ Avoid: Unclear names and missing documentation
pub async fn send(a: &mut App) -> Result<Option<String>> {
    // ... implementation
}
```

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

Examples:
```
feat: add support for Bearer token authentication
fix: prevent crash when terminal is too small
docs: update keyboard shortcuts in README
refactor: extract UI components into separate modules
```

## 🧪 Testing

### Test Categories

1. **Unit Tests**: Test individual functions and modules
2. **Integration Tests**: Test component interactions
3. **UI Tests**: Test terminal UI rendering (no-crash tests)
4. **Network Tests**: Test HTTP functionality (marked with `#[ignore]`)

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_output);
    }
    
    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_network_functionality() {
        // Network tests should be ignored by default
    }
}
```

### Running Tests

```bash
# Quick tests (no network)
cargo test

# All tests including network
cargo test -- --ignored

# Specific test
cargo test test_name

# Test with output
cargo test -- --nocapture
```

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Environment**:
   - Operating system and version
   - Terminal emulator and version
   - Rust version (`rustc --version`)

2. **Steps to reproduce**:
   - Clear, step-by-step instructions
   - Expected vs actual behavior

3. **Logs/Screenshots**:
   - Error messages
   - Screenshots if UI-related

### Bug Report Template

```markdown
**Environment:**
- OS: [e.g., Ubuntu 22.04, macOS 13.0, Windows 11]
- Terminal: [e.g., Alacritty 0.12.0, iTerm2 3.4.19]
- Rust: [output of `rustc --version`]

**Description:**
A clear description of the bug.

**Steps to Reproduce:**
1. Start restless
2. Navigate to...
3. Press key...
4. See error

**Expected Behavior:**
What you expected to happen.

**Actual Behavior:**
What actually happened.

**Logs/Screenshots:**
Include any relevant error messages or screenshots.
```

## 💡 Feature Requests

We love new ideas! When suggesting features:

1. **Check existing issues** to avoid duplicates
2. **Describe the problem** you're trying to solve
3. **Explain your proposed solution**
4. **Consider alternatives** you've thought about
5. **Think about implementation** complexity

### Feature Request Template

```markdown
**Problem:**
What problem does this feature solve?

**Proposed Solution:**
How should this feature work?

**Alternatives:**
What other solutions have you considered?

**Additional Context:**
Mockups, examples, or other relevant information.
```

## 🎨 UI/UX Guidelines

### Terminal UI Principles

1. **Keyboard-First**: All functionality should be accessible via keyboard
2. **Responsive**: Adapt gracefully to different terminal sizes
3. **Intuitive**: Follow common terminal UI conventions
4. **Fast**: Minimize input latency and screen updates
5. **Accessible**: Support different terminal capabilities

### Color and Styling

- Use the project's color constants from `ui/mod.rs`
- Test with different terminal color schemes
- Ensure good contrast for readability
- Use colors consistently (e.g., green for success, red for errors)

## 📚 Documentation

### Code Documentation

```rust
/// Brief description of the function
///
/// Longer description if needed, explaining the purpose,
/// behavior, and any important details.
///
/// # Arguments
///
/// * `param1` - Description of parameter
/// * `param2` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// When this function returns an error and why
///
/// # Examples
///
/// ```rust
/// let result = function_name(arg1, arg2)?;
/// assert_eq!(result, expected);
/// ```
pub fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // Implementation
}
```

### README Updates

When adding features:
1. Update the features list
2. Add keyboard shortcuts if applicable
3. Update screenshots if UI changed
4. Add usage examples

## 🏷️ Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- Major: Breaking changes
- Minor: New features (backward compatible)
- Patch: Bug fixes (backward compatible)

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Git tag created
- [ ] Release notes written

## 🤝 Code of Conduct

### Our Pledge

We are committed to making participation in this project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity and expression, level of experience, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

Examples of behavior that contributes to a positive environment:
- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the project team. All complaints will be reviewed and investigated promptly and fairly.

## 🆘 Getting Help

- **Documentation**: Check the README and inline documentation
- **Issues**: Search existing issues or create a new one
- **Discussions**: Use GitHub Discussions for questions
- **Code**: Read the source code and comments

## 🎉 Recognition

Contributors will be recognized in:
- Git commit history
- Release notes
- Contributors section (coming soon)

Thank you for contributing to Restless! 🚀