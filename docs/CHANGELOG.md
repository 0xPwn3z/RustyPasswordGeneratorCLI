# Changelog

All notable changes to the Rusty Password Generator project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Password strength meter
- Multiple password generation at once
- Export to file functionality
- Custom character set input
- Password entropy calculator

## [0.1.0] - 2025-11-12

### Added
- Initial release of Rusty Password Generator
- Command-line interface for password generation
- Customizable password length (8-128 characters)
- Optional character sets:
  - Lowercase letters (a-z) - always included
  - Uppercase letters (A-Z) - optional via `-u` flag
  - Numbers (0-9) - optional via `-n` flag
  - Special characters (!@#$%^&*_-+=<>?) - optional via `-s` flag
- Password cracking time estimation based on bcrypt speed
- Colorful ASCII art logo using FIGfont
- Human-readable time duration formatting
- Comprehensive documentation:
  - Inline code documentation with doc comments
  - README.md with usage examples
  - CONTRIBUTING.md for contributors
  - API.md with detailed technical documentation
  - CHANGELOG.md for version tracking

### Security
- Uses cryptographically secure random number generation via `rand` crate
- Shuffles character set before each character selection for enhanced randomness
- Conservative bcrypt cracking speed assumption (9,000 attempts/sec)

### Dependencies
- `rand` - Random number generation
- `clap` - Command-line argument parsing
- `figlet-rs` - ASCII art generation
- `colored` - Terminal text coloring
- `humantime` - Duration formatting

### Technical Details
- Written in Rust 🦀
- Single-file architecture (src/main.rs)
- Well-structured code with clear separation of concerns
- Constants for configuration values
- Comprehensive error handling with user-friendly messages
- Input validation for password length

## Version History

### Format
```
[Version] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security improvements or fixes
```

---

## Future Roadmap

### v0.2.0 (Planned)
- Password strength scoring system
- Batch generation mode
- Custom character sets via command-line
- Optional clipboard integration
- Configuration file support

### v0.3.0 (Planned)
- Password pattern validation
- Entropy calculator
- Pronounceable password option
- Password manager integration
- JSON output format

### v1.0.0 (Planned)
- Stable API
- Comprehensive test suite
- Performance optimizations
- Complete documentation
- Binary releases for major platforms

---

*Note: This changelog follows [Keep a Changelog](https://keepachangelog.com/) principles.*

