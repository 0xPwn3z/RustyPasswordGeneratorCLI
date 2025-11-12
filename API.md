# API Documentation

This document provides detailed technical documentation for the Rusty Password Generator codebase.

## Table of Contents

- [Overview](#overview)
- [Module Structure](#module-structure)
- [Constants](#constants)
- [Types](#types)
- [Functions](#functions)
- [Algorithms](#algorithms)
- [Dependencies](#dependencies)

## Overview

The Rusty Password Generator is a single-file CLI application that generates secure random passwords with customizable options. The code is organized into logical sections with clear separation of concerns.

## Module Structure

```
src/main.rs
├── Module Documentation (//!)
├── Imports
├── Constants Section
├── Types Section (Args struct)
├── Main Entry Point
└── Helper Functions
```

## Constants

### `DEFAULT_LENGTH: u32 = 16`

**Purpose**: Default password length when user doesn't specify one.

**Type**: `u32` (unsigned 32-bit integer)

**Value**: `16` characters

**Rationale**: 16 characters provides a good balance between security and usability for most applications.

---

### `MIN_LENGTH: u32 = 8`

**Purpose**: Minimum allowed password length.

**Type**: `u32`

**Value**: `8` characters

**Rationale**: Passwords shorter than 8 characters are generally considered weak, even with a complex character set.

---

### `MAX_LENGTH: u32 = 128`

**Purpose**: Maximum allowed password length.

**Type**: `u32`

**Value**: `128` characters

**Rationale**: Provides an upper bound to prevent excessive memory usage while still allowing very strong passwords.

---

### `CHARS: &str`

**Purpose**: Base character set containing lowercase letters.

**Type**: `&'static str` (string slice with static lifetime)

**Value**: `"abcdefghijklmnopqrstuvwxyz"`

**Usage**: Always included in the character set (26 characters)

---

### `UPPERCASE_CHARS: &str`

**Purpose**: Uppercase letter characters.

**Type**: `&'static str`

**Value**: `"ABCDEFGHIJKLMNOPQRSTUVWXYZ"`

**Usage**: Included when `--uppercase-chars` flag is set (26 characters)

---

### `NUMBERS: &str`

**Purpose**: Numeric digit characters.

**Type**: `&'static str`

**Value**: `"0123456789"`

**Usage**: Included when `--numbers` flag is set (10 characters)

---

### `SPECIAL_CHARS: &str`

**Purpose**: Special symbol characters for enhanced security.

**Type**: `&'static str`

**Value**: `"!@#$%^&*_-+=<>?"`

**Usage**: Included when `--special-chars` flag is set (15 characters)

---

### `BCRYPT_CRACKING_SPEED: u128`

**Purpose**: Assumed cracking speed for security estimation.

**Type**: `u128` (unsigned 128-bit integer)

**Value**: `9*(10^3)` = 9,000 attempts per second

**Rationale**: Conservative estimate for bcrypt hash cracking on modern hardware. Bcrypt is intentionally slow to resist brute-force attacks.

**Note**: This is a theoretical value. Actual cracking speeds vary based on hardware and bcrypt cost factor.

## Types

### `Args` Struct

**Purpose**: Represents parsed command-line arguments.

**Derives**: `Parser`, `Debug`

**Attributes**: Uses `clap` macros for argument parsing

#### Fields

```rust
struct Args {
    length: u32,              // Password length
    uppercase_chars: bool,    // Include uppercase flag
    special_chars: bool,      // Include special chars flag
    numbers: bool,            // Include numbers flag
}
```

#### Field Details

| Field | Type | Default | CLI Short | CLI Long | Description |
|-------|------|---------|-----------|----------|-------------|
| `length` | `u32` | `16` | `-l` | `--length` | Password length (8-128) |
| `uppercase_chars` | `bool` | `false` | `-u` | `--uppercase-chars` | Include A-Z |
| `special_chars` | `bool` | `false` | `-s` | `--special-chars` | Include symbols |
| `numbers` | `bool` | `false` | `-n` | `--numbers` | Include 0-9 |

## Functions

### `main()`

**Signature**: `fn main()`

**Purpose**: Application entry point and orchestration.

**Returns**: `()` (void)

**Flow**:
```
1. print_logo()
2. Args::parse()
3. set_length(&args)
4. create_charset(&args)
5. compute_password(length, &charset)
6. compute_time_to_crack(charset_size, length)
7. println!(results)
```

**Side Effects**: 
- Prints to stdout
- Parses command-line arguments
- Exits on argument parsing errors (handled by clap)

---

### `print_logo()`

**Signature**: `fn print_logo()`

**Purpose**: Display ASCII art banner for the application.

**Returns**: `()` (void)

**Algorithm**:
1. Load standard FIGfont
2. Convert "Rusty" to ASCII art
3. Convert "Password Generator" to ASCII art
4. Print with green coloring (using `colored` crate)
5. Fallback to simple text if conversion fails

**Dependencies**: `figlet_rs`, `colored`

**Error Handling**: Uses fallback text if FIGfont conversion returns `None`

---

### `set_length(args: &Args) -> u32`

**Signature**: `fn set_length(args: &Args) -> u32`

**Purpose**: Validate and sanitize password length input.

**Parameters**:
- `args: &Args` - Reference to parsed command-line arguments

**Returns**: `u32` - Validated password length

**Algorithm**:
```rust
if args.length < MIN_LENGTH || args.length > MAX_LENGTH {
    // Print warning
    return DEFAULT_LENGTH;
} else {
    return args.length;
}
```

**Validation Rules**:
- Minimum: 8 characters
- Maximum: 128 characters
- Default fallback: 16 characters

**Side Effects**: Prints warning message if invalid length provided

---

### `create_charset(args: &Args) -> String`

**Signature**: `fn create_charset(args: &Args) -> String`

**Purpose**: Build the character pool for password generation.

**Parameters**:
- `args: &Args` - Reference to parsed command-line arguments

**Returns**: `String` - Concatenated string of all available characters

**Algorithm**:
```
1. Start with CHARS (lowercase a-z)
2. If uppercase_chars flag: append UPPERCASE_CHARS
3. If special_chars flag: append SPECIAL_CHARS
4. If numbers flag: append NUMBERS
5. Return combined string
```

**Character Set Sizes**:
| Configuration | Size | Example |
|---------------|------|---------|
| Lowercase only | 26 | `a-z` |
| + Uppercase | 52 | `a-zA-Z` |
| + Numbers | 62 | `a-zA-Z0-9` |
| + Special | 77 | `a-zA-Z0-9!@#...` |

**Note**: Order of concatenation doesn't affect security as characters are randomly selected.

---

### `compute_password(length: u32, charset: &String) -> String`

**Signature**: `fn compute_password(length: u32, charset: &String) -> String`

**Purpose**: Generate a random password from the given character set.

**Parameters**:
- `length: u32` - Desired password length
- `charset: &String` - String containing all available characters

**Returns**: `String` - Randomly generated password

**Algorithm**:
```rust
1. Initialize RNG (rand::rng())
2. Convert charset to Vec<char>
3. For each position (0..length):
   a. Shuffle the charset vector
   b. Choose random character
   c. Append to password string
4. Return password
```

**Randomness**:
- Uses `rand` crate's default RNG
- Each character position: shuffle then choose
- Double randomization for enhanced security

**Time Complexity**: O(n × m) where:
- n = password length
- m = charset size (for shuffling)

**Space Complexity**: O(m) for charset vector

**Dependencies**: `rand::prelude::*`

---

### `compute_time_to_crack(charset_length: u32, pwd_length: u32) -> u128`

**Signature**: `fn compute_time_to_crack(charset_length: u32, pwd_length: u32) -> u128`

**Purpose**: Estimate time to crack password via brute-force attack.

**Parameters**:
- `charset_length: u32` - Number of unique characters in character set
- `pwd_length: u32` - Length of the password

**Returns**: `u128` - Estimated seconds to crack

**Algorithm**:
```
Total attempts = Σ(charset_length^i) for i=1 to pwd_length
Time (seconds) = Total attempts / BCRYPT_CRACKING_SPEED
```

**Mathematical Formula**:
```
T = (Σ[i=1 to n] c^i) / s

Where:
  T = Time to crack (seconds)
  c = charset length
  n = password length
  s = cracking speed (attempts/sec)
```

**Example Calculation**:
```
Charset: a-zA-Z0-9 (62 characters)
Length: 8 characters
Speed: 9,000 attempts/sec

Total = 62^1 + 62^2 + ... + 62^8
      = 221,919,451,578,090
Time = 221,919,451,578,090 / 9,000
     ≈ 24,657,716,842 seconds
     ≈ 781 years
```

**Assumptions**:
- Brute-force attack (tries all combinations)
- Bcrypt hashing (9,000 attempts/sec)
- No rainbow tables or dictionary attacks
- No hardware acceleration beyond standard CPU

**Type Notes**: Uses `u128` to handle extremely large numbers for strong passwords.

## Algorithms

### Password Generation Algorithm

**Name**: Shuffled Random Selection

**Type**: Non-deterministic, cryptographically secure (depends on RNG)

**Steps**:
1. Create character pool from user options
2. For each character position:
   - Shuffle entire character pool
   - Randomly select one character
   - Add to password string

**Properties**:
- **Uniform Distribution**: Each character has equal probability
- **Independence**: Each position is independent
- **No Repetition Prevention**: Characters can repeat (desired for entropy)

**Security**: Relies on `rand` crate's CSPRNG (Cryptographically Secure Pseudo-Random Number Generator)

### Cracking Time Estimation Algorithm

**Name**: Exhaustive Search Time Calculation

**Type**: Deterministic mathematical calculation

**Basis**: Calculates total keyspace and divides by attack speed

**Formula Derivation**:
```
For password of length n with charset c:
- 1-char passwords: c^1
- 2-char passwords: c^2
- ...
- n-char passwords: c^n

Total = c^1 + c^2 + ... + c^n
      = c(c^n - 1)/(c - 1)  [geometric series]
```

**Approximation**: For large n and c, dominated by c^n term.

## Dependencies

### Runtime Dependencies

#### `rand` (v0.8+)
- **Purpose**: Random number generation
- **Usage**: `rand::rng()`, `shuffle()`, `choose()`
- **Security**: Provides cryptographically secure randomness

#### `clap` (v4+)
- **Purpose**: Command-line argument parsing
- **Usage**: `#[derive(Parser)]`, automatic help generation
- **Features**: Type-safe, auto-generated help/version

#### `figlet-rs`
- **Purpose**: ASCII art text generation
- **Usage**: `FIGfont::standard()`, `convert()`
- **Features**: Multiple fonts, fallback handling

#### `colored`
- **Purpose**: Terminal text coloring
- **Usage**: `.green()` method on strings
- **Features**: Cross-platform color support

#### `humantime`
- **Purpose**: Human-readable duration formatting
- **Usage**: `format_duration()`
- **Features**: Converts seconds to "X years Y days" format

### Build Dependencies

Standard Rust toolchain:
- `rustc` - Rust compiler
- `cargo` - Build system and package manager

## Performance Characteristics

### Time Complexity

| Function | Best Case | Average Case | Worst Case |
|----------|-----------|--------------|------------|
| `main()` | O(n×m) | O(n×m) | O(n×m) |
| `set_length()` | O(1) | O(1) | O(1) |
| `create_charset()` | O(m) | O(m) | O(m) |
| `compute_password()` | O(n×m) | O(n×m) | O(n×m) |
| `compute_time_to_crack()` | O(n) | O(n) | O(n) |

Where:
- n = password length
- m = charset size

### Space Complexity

| Function | Space |
|----------|-------|
| `main()` | O(n + m) |
| `create_charset()` | O(m) |
| `compute_password()` | O(n + m) |
| `compute_time_to_crack()` | O(1) |

## Security Considerations

### Random Number Generation
- Uses system entropy for seeding
- CSPRNG ensures unpredictable output
- No observable patterns in generated passwords

### Character Distribution
- Uniform probability across charset
- Shuffle algorithm prevents bias
- No sequential patterns

### Side-Channel Attacks
- Timing: Password generation time varies with length (acceptable for CLI)
- Memory: No sensitive data left in memory longer than necessary
- Output: Password displayed once, not logged

### Limitations
- Password not cleared from memory after display
- No clipboard integration (reduces attack surface)
- Time-to-crack is theoretical estimate only

## Error Handling

### Current Approach
- Invalid length: Warning + fallback to default
- FIGfont failure: Fallback to simple text
- Argument parsing: Handled by `clap` (auto-exits with help)

### Potential Errors (Not Currently Handled)
- System RNG failure: Would panic (extremely rare)
- Terminal color support: `colored` handles gracefully
- Memory allocation: Would panic on OOM

## Future Enhancement Opportunities

1. **Password Validation**: Ensure at least one char from each selected category
2. **Batch Generation**: Generate multiple passwords at once
3. **Custom Charset**: Allow user-provided character sets
4. **Password Scoring**: Real-time strength meter
5. **Clipboard Integration**: Copy to clipboard option
6. **Secure Memory**: Zero password from memory after display
7. **Export Options**: Save to file (encrypted)
8. **Pattern Avoidance**: Prevent common patterns
9. **Pronounceable Passwords**: Option for easier memorization
10. **Integration**: Plugin for password managers

## Testing Strategy

### Unit Tests (Recommended)
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_length_validation() { ... }
    
    #[test]
    fn test_charset_creation() { ... }
    
    #[test]
    fn test_password_length() { ... }
    
    #[test]
    fn test_charset_usage() { ... }
}
```

### Integration Tests
- CLI argument parsing
- Full password generation flow
- Edge cases (min/max length)

### Property-Based Tests
- Password always matches requested length
- Characters always from specified charset
- No crashes with any valid input

---

*Last Updated: November 2025*

