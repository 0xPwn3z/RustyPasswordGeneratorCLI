# Quick Start Guide

Get started with Rusty Password Generator in under 5 minutes! 🚀

## 📥 Installation

### Prerequisites
- Rust 1.70 or later ([Install Rust](https://rustup.rs/))

### Clone & Build
```bash
git clone <your-repo-url>
cd CLIPasswordGenerator
cargo build --release
```

## 🎯 First Password

Generate your first password with defaults (16 characters, lowercase only):

```bash
cargo run
```

**Output:**
```
 ____            _         
|  _ \ _   _ ___| |_ _   _ 
...

Generated Password: kxmqpzwvjhbytnal
Cracking Time (bcrypt speed assumption: 9*(10^3) attempt/sec): 3 days
```

## 🔐 Common Use Cases

### Personal Account (Medium Security)
20 characters with uppercase and numbers:
```bash
cargo run -- -l 20 -u -n
```

### High Security Account
32 characters with all options:
```bash
cargo run -- -l 32 -u -n -s
```

### Maximum Security
128 characters with everything:
```bash
cargo run -- -l 128 -u -n -s
```

### Simple PIN Alternative
8 numbers only:
```bash
cargo run -- -l 8 -n
```

## 📝 Command Cheat Sheet

| Command | Effect |
|---------|--------|
| `cargo run` | Default 16-char password |
| `cargo run -- -l 24` | 24-character password |
| `cargo run -- -u` | Include uppercase (A-Z) |
| `cargo run -- -n` | Include numbers (0-9) |
| `cargo run -- -s` | Include symbols (!@#$...) |
| `cargo run -- --help` | Show all options |

## 🎨 Understanding the Output

```
Generated Password: aB3#xK9$
Cracking Time: 2 years 3 months
```

- **Generated Password**: Your new secure password
- **Cracking Time**: Estimated time to crack via brute-force attack on bcrypt hash

## 💡 Pro Tips

1. **Save to File**: Redirect output to save password
   ```bash
   cargo run -- -l 32 -u -n -s > password.txt
   ```

2. **Build Once, Use Many Times**: After building, use the binary directly
   ```bash
   cargo build --release
   ./target/release/CLIPasswordGenerator -l 20 -u -n
   ```

3. **Add to PATH**: For system-wide access
   ```bash
   sudo cp target/release/CLIPasswordGenerator /usr/local/bin/pwdgen
   pwdgen -l 24 -u -n -s
   ```

4. **Combine with Other Tools**: Pipe to clipboard (macOS)
   ```bash
   cargo run -- -l 20 -u -n | grep "Generated" | awk '{print $3}' | pbcopy
   ```

## 🔒 Security Best Practices

✅ **DO:**
- Use at least 16 characters for important accounts
- Include uppercase, numbers, and symbols for maximum security
- Use unique passwords for each account
- Store passwords in a password manager

❌ **DON'T:**
- Reuse passwords across accounts
- Use dictionary words or personal information
- Share passwords via insecure channels
- Use passwords shorter than 8 characters

## 🆘 Troubleshooting

### "Password length must be between 8 and 128"
You specified a length outside the valid range. Use `-l` with a value between 8 and 128.

### Build Errors
Update Rust to the latest version:
```bash
rustup update
```

### Colors Not Showing
Your terminal may not support colors. The app will still work with plain text.

## 📚 Learn More

- [Full Documentation](../README.md)
- [API Reference](API.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

## 🎓 Examples by Scenario

### Banking/Financial
```bash
cargo run -- -l 32 -u -n -s
# Strong, long password with all character types
```

### Email Account
```bash
cargo run -- -l 24 -u -n
# Balanced security without special chars (if not allowed)
```

### Social Media
```bash
cargo run -- -l 20 -u -n -s
# Good security for frequently accessed accounts
```

### Local Development
```bash
cargo run -- -l 16 -u
# Moderate security for development environments
```

### Guest WiFi
```bash
cargo run -- -l 12 -n
# Simple but secure for temporary access
```

## 🚀 Next Steps

1. ⭐ Star the repository
2. 📖 Read the full [README](../README.md)
3. 🤝 [Contribute](CONTRIBUTING.md) improvements
4. 🐛 Report issues on GitHub
5. 💬 Share with friends!

## 📞 Need Help?

- Check [README.md](../README.md) for detailed usage
- See [API.md](API.md) for technical details
- Open an issue on GitHub
- Read [CONTRIBUTING.md](CONTRIBUTING.md) for development help

---

**Happy password generating! 🎉**

