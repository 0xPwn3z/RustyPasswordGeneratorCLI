# Contributing to Rusty Password Generator

First off, thank you for considering contributing to Rusty Password Generator! It's people like you that make this tool better for everyone.

## 🤝 Code of Conduct

This project and everyone participating in it is governed by a code of respect and professionalism. By participating, you are expected to uphold this code.

## 🐛 How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples** (command-line arguments used)
- **Describe the behavior you observed** and what you expected
- **Include your environment details**:
  - OS version
  - Rust version (`rustc --version`)
  - Cargo version (`cargo --version`)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Use a clear and descriptive title**
- **Provide a step-by-step description** of the suggested enhancement
- **Provide specific examples** to demonstrate the enhancement
- **Explain why this enhancement would be useful**

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. Ensure your code follows the existing style
4. Make sure your code compiles without warnings
5. Write a clear commit message

## 🏗️ Development Process

### Setting Up Your Development Environment

```bash
# Clone your fork
git clone https://github.com/your-username/CLIPasswordGenerator.git
cd CLIPasswordGenerator

# Create a branch for your feature
git checkout -b feature/amazing-feature

# Install dependencies
cargo build
```

### Code Style Guidelines

This project follows standard Rust conventions:

- **Formatting**: Use `rustfmt` to format your code
  ```bash
  cargo fmt
  ```

- **Linting**: Run `clippy` to catch common mistakes
  ```bash
  cargo clippy
  ```

- **Documentation**: 
  - Add doc comments (`///`) for public functions
  - Add inline comments for complex logic
  - Update README.md if adding new features

### Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests when relevant

Examples:
```
Add support for custom character sets
Fix password length validation bug
Update documentation for new CLI options
Refactor charset creation logic
```

### Testing Your Changes

```bash
# Run the project
cargo run

# Test with different options
cargo run -- -l 24 -u -n -s
cargo run -- --length 8
cargo run -- --help

# Build in release mode
cargo build --release
```

### Documentation

When adding or modifying features:

1. Update inline code documentation
2. Update README.md if user-facing changes
3. Add examples for new features
4. Update help text in `Args` struct if needed

## 📝 Code Review Process

1. A maintainer will review your pull request
2. They may suggest changes or improvements
3. Once approved, a maintainer will merge your PR
4. Your contribution will be included in the next release!

## 🎯 Areas for Contribution

Looking for ideas? Here are some areas that could use help:

### Easy Issues (Good First Issues)
- Improve error messages
- Add more examples to README
- Fix typos in documentation
- Add unit tests

### Medium Issues
- Add password strength meter
- Support for generating multiple passwords
- Add entropy calculation
- Custom character set input

### Advanced Issues
- Password pattern validation
- Integration with system keychain
- Encrypted password storage
- Advanced security metrics

## 🔍 Code Structure

### Main Components

```rust
// Constants - Configuration values
const DEFAULT_LENGTH: u32 = 16;
const BCRYPT_CRACKING_SPEED: u128 = 9*(10^3);

// Args - Command-line argument structure
struct Args { ... }

// Main flow
fn main() {
    print_logo();           // Display ASCII art
    let args = ...;         // Parse arguments
    let length = ...;       // Validate length
    let charset = ...;      // Build character set
    let password = ...;     // Generate password
    let time = ...;         // Calculate security
}

// Helper functions
fn print_logo() { ... }
fn set_length() { ... }
fn create_charset() { ... }
fn compute_password() { ... }
fn compute_time_to_crack() { ... }
```

### Adding a New Feature

Example: Adding a minimum uppercase letter requirement

```rust
// 1. Add to Args struct
#[derive(Parser, Debug)]
struct Args {
    // ...existing fields...
    
    /// Minimum number of uppercase letters required
    #[arg(long, default_value_t = 0)]
    min_uppercase: u32,
}

// 2. Implement validation in a new or existing function
fn validate_password(password: &str, args: &Args) -> bool {
    let uppercase_count = password.chars()
        .filter(|c| c.is_uppercase())
        .count() as u32;
    uppercase_count >= args.min_uppercase
}

// 3. Update main() to use the new feature
fn main() {
    // ...existing code...
    let mut password = compute_password(length, &charset);
    
    // Regenerate if validation fails
    while !validate_password(&password, &args) {
        password = compute_password(length, &charset);
    }
    
    // ...rest of code...
}

// 4. Update documentation
// - Add doc comments
// - Update README.md
// - Add usage examples
```

## 🧪 Testing Checklist

Before submitting a PR, verify:

- [ ] Code compiles without errors (`cargo build`)
- [ ] No warnings from clippy (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation is updated
- [ ] Examples work as described
- [ ] Edge cases are handled (min/max length, etc.)

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [clap Documentation](https://docs.rs/clap/latest/clap/)

## ❓ Questions?

Feel free to open an issue with the "question" label if you need help or clarification on anything!

## 🎉 Recognition

Contributors will be recognized in:
- README.md Contributors section
- Release notes
- Project documentation

Thank you for contributing! 🚀

