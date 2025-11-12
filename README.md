# Rusty Password Generator 🔐

A secure, feature-rich command-line password generator written in Rust.

## 🌟 Features

- **Customizable Length**: Generate passwords from 8 to 128 characters
- **Flexible Character Sets**: Choose from:
  - Lowercase letters (a-z) - always included
  - Uppercase letters (A-Z) - optional
  - Numbers (0-9) - optional
  - Special characters (!@#$%^&*_-+=<>?) - optional
- **Security Estimation**: Estimates time to crack password based on bcrypt attack speed
- **Beautiful CLI**: Colorful ASCII art logo and formatted output
- **Fast & Efficient**: Written in Rust for performance and safety

## 📋 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)
- Cargo (comes with Rust)

## 🚀 Installation

Clone the repository and build:

```bash
git clone <your-repo-url>
cd CLIPasswordGenerator
cargo build --release
```

## 💻 Usage

### Basic Usage

Generate a default 16-character password with lowercase letters only:

```bash
cargo run
```

### Advanced Usage

Generate a custom password with specific options:

```bash
# 24-character password with all character types
cargo run -- -l 24 -u -n -s

# 32-character password with uppercase and numbers
cargo run -- --length 32 --uppercase-chars --numbers

# Maximum security: 128-character password with all options
cargo run -- -l 128 -u -n -s
```

### Command-Line Options

| Option | Short | Long | Description | Default |
|--------|-------|------|-------------|---------|
| Length | `-l` | `--length` | Password length (8-128) | 16 |
| Uppercase | `-u` | `--uppercase-chars` | Include A-Z | false |
| Numbers | `-n` | `--numbers` | Include 0-9 | false |
| Special | `-s` | `--special-chars` | Include !@#$%^&*_-+=<>? | false |

### Examples

```bash
# Simple 12-character lowercase password
cargo run -- -l 12

# Strong 20-character password with uppercase and numbers
cargo run -- -l 20 -u -n

# Maximum security password
cargo run -- -l 64 -u -n -s
```

## 📖 How It Works

1. **Argument Parsing**: Uses `clap` to parse command-line arguments
2. **Validation**: Ensures password length is within valid bounds (8-128)
3. **Character Set Building**: Constructs a pool of characters based on user options
4. **Random Generation**: Uses cryptographically secure random number generation
5. **Security Estimation**: Calculates estimated cracking time based on:
   - Character set size
   - Password length
   - Assumed bcrypt cracking speed (9,000 attempts/sec)

## 🔧 Dependencies

- **rand** - Cryptographically secure random number generation
- **clap** - Command-line argument parsing
- **figlet-rs** - ASCII art text generation
- **colored** - Terminal text coloring
- **humantime** - Human-readable duration formatting

## 📁 Project Structure

```
CLIPasswordGenerator/
├── Cargo.toml          # Project dependencies and metadata
├── Cargo.lock          # Dependency lock file
├── README.md           # This file
└── src/
    └── main.rs         # Main application code
```

## 🛡️ Security Considerations

- Uses Rust's `rand` crate for random number generation
- Password cracking time estimates are theoretical and assume:
  - Brute-force attack against bcrypt hashes
  - 9,000 attempts per second (conservative estimate)
  - No use of rainbow tables or dictionary attacks
- Real-world security depends on proper password storage (hashing, salting)

## 🎨 Output Example

```
 ____            _         
|  _ \ _   _ ___| |_ _   _ 
| |_) | | | / __| __| | | |
|  _ <| |_| \__ \ |_| |_| |
|_| \_\\__,_|___/\__|\__, |
                     |___/ 

 ____                                     _ 
|  _ \ __ _ ___ _____      _____  _ __ __| |
| |_) / _` / __/ __\ \ /\ / / _ \| '__/ _` |
|  __/ (_| \__ \__ \\ V  V / (_) | | | (_| |
|_|   \__,_|___/___/ \_/\_/ \___/|_|  \__,_|

  ____                           _             
 / ___| ___ _ __   ___ _ __ __ _| |_ ___  _ __ 
| |  _ / _ \ '_ \ / _ \ '__/ _` | __/ _ \| '__|
| |_| |  __/ | | |  __/ | | (_| | || (_) | |   
 \____|\___|_| |_|\___|_|  \__,_|\__\___/|_|   

Generated Password: aB3#xK9$mN2@pQ7&vL4!
Cracking Time (bcrypt speed assumption: 9*(10^3) attempt/sec): 2 years 3 months 15 days
```

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

## 👤 Author

Created with ❤️ using Rust

## 🔮 Future Enhancements

Potential features for future versions:
- [ ] Password strength meter
- [ ] Multiple password generation at once
- [ ] Save passwords to file (encrypted)
- [ ] Password entropy calculator
- [ ] Custom character set definition
- [ ] Password pattern validation
- [ ] Integration with password managers

## 📚 Learn More

- [Rust Programming Language](https://www.rust-lang.org/)
- [Password Security Best Practices](https://www.owasp.org/index.php/Authentication_Cheat_Sheet)
- [Bcrypt Algorithm](https://en.wikipedia.org/wiki/Bcrypt)

