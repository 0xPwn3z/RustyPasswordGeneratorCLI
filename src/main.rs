//! # Rusty Password Generator
//!
//! A command-line interface (CLI) tool for generating secure random passwords with customizable options.
//!
//! ## Features
//! - Generate passwords with customizable length (8-128 characters)
//! - Optional inclusion of uppercase letters, numbers, and special characters
//! - Estimates time to crack the password using bcrypt speed assumptions
//! - Colorful ASCII art logo display
//!
//! ## Usage
//! ```bash
//! # Generate a default 16-character password
//! cargo run
//!
//! # Generate a 24-character password with uppercase, numbers, and special characters
//! cargo run -- -l 24 -u -n -s
//! ```

use rand::prelude::*;
use clap::Parser;
use figlet_rs::FIGfont;
use colored::*;
use humantime::format_duration;
use std::time::Duration;

// ============================================================================
// Constants
// ============================================================================

/// Default password length when not specified by the user
const DEFAULT_LENGTH: u32 = 16;

/// Minimum allowed password length
const MIN_LENGTH: u32 = 8;

/// Maximum allowed password length
const MAX_LENGTH: u32 = 128;

/// Lowercase alphabet characters
const CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

/// Uppercase alphabet characters
const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Numeric digits
const NUMBERS: &str = "0123456789";

/// Special characters for enhanced password security
const SPECIAL_CHARS: &str = "!@#$%^&*_-+=<>?";

/// Assumed bcrypt cracking speed in attempts per second (9,000 attempts/sec)
/// This is a conservative estimate for modern hardware
const BCRYPT_CRACKING_SPEED: u128 = 9*(10^3);

// ============================================================================
// Command-Line Arguments
// ============================================================================

/// Command-line arguments structure for password generation configuration
///
/// Uses the `clap` crate to parse command-line arguments with sensible defaults.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Password length (must be between 8 and 128 characters)
    #[arg(short, long, default_value_t = DEFAULT_LENGTH)]
    length: u32,

    /// Include uppercase characters (A-Z) in the password
    #[arg(short, long, default_value_t = false)]
    uppercase_chars: bool,

    /// Include special characters (!@#$%^&*_-+=<>?) in the password
    #[arg(short, long, default_value_t = false)]
    special_chars: bool,

    /// Include numeric digits (0-9) in the password
    #[arg(short, long, default_value_t = false)]
    numbers: bool,
}

// ============================================================================
// Main Entry Point
// ============================================================================

/// Main entry point for the password generator application
///
/// # Workflow
/// 1. Display the application logo
/// 2. Parse command-line arguments
/// 3. Validate and set password length
/// 4. Create character set based on user preferences
/// 5. Generate the random password
/// 6. Calculate estimated time to crack
/// 7. Display results to the user
fn main() {
    // Display the ASCII art logo
    print_logo();

    // Parse command-line arguments
    let args = Args::parse();

    // Validate and set the password length
    let length = set_length(&args);

    // Build the character set based on user options
    let charset = create_charset(&args);

    // Generate the random password
    let password = compute_password(length, &charset);

    // Calculate estimated cracking time
    let cracking_speed = compute_time_to_crack(charset.chars().count() as u32, length);
    let cs = Duration::from_secs(cracking_speed as u64);

    // Display the generated password and security information
    println!("Generated Password: {}\nCracking Time (bcrypt speed assumption: 9*(10^3) attempt/sec): {}", password, format_duration(cs));
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Prints the application logo using ASCII art
///
/// Displays "Rusty Password Generator" in green using FIGfont.
/// Falls back to simple text if FIGfont conversion fails.
fn print_logo() {
    // Load the standard FIGfont
    let standard_font = FIGfont::standard().unwrap();

    // Convert "Rusty" to ASCII art
    let figure = standard_font.convert("Rusty");

    // Convert "Password Generator" to ASCII art
    let figure1 = standard_font.convert("Password Generator");

    // Display "Rusty" (with fallback)
    match figure {
        Some(f) => {
            println!("{}", f.to_string().green());
        }
        None => {
            println!("[+] Rusty [+]");
        }
    }

    // Display "Password Generator" (with fallback)
    match figure1 {
        Some(f) => {
            println!("{}", f.to_string().green());
        }
        None => {
            println!("[>] Password Generator [<]");
        }
    }
}

/// Validates and sets the password length within acceptable bounds
///
/// # Arguments
/// * `args` - Reference to the parsed command-line arguments
///
/// # Returns
/// * `u32` - Validated password length (defaults to DEFAULT_LENGTH if out of bounds)
///
/// # Notes
/// If the requested length is outside the range [MIN_LENGTH, MAX_LENGTH],
/// a warning is printed and DEFAULT_LENGTH is used instead.
fn set_length (args: &Args) -> u32 {
    let mut length = args.length;

    // Check if length is within valid bounds
    if args.length < MIN_LENGTH || args.length > MAX_LENGTH {
        println!("Password length must be between {} and {}.", MIN_LENGTH, MAX_LENGTH);
        length = DEFAULT_LENGTH;
    }

    length
}

/// Creates a character set based on user-specified options
///
/// # Arguments
/// * `args` - Reference to the parsed command-line arguments
///
/// # Returns
/// * `String` - Character set to use for password generation
///
/// # Character Set Composition
/// - Always includes lowercase letters (a-z)
/// - Optionally includes uppercase letters (A-Z)
/// - Optionally includes special characters (!@#$%^&*_-+=<>?)
/// - Optionally includes numbers (0-9)
fn create_charset(args: &Args) -> String {
    // Start with lowercase letters as the base character set
    let mut charset = String::from(CHARS);

    // Add uppercase letters if requested
    if args.uppercase_chars {
        charset.push_str(UPPERCASE_CHARS);
    }

    // Add special characters if requested
    if args.special_chars {
        charset.push_str(SPECIAL_CHARS);
    }

    // Add numbers if requested
    if args.numbers {
        charset.push_str(NUMBERS);
    }

    charset
}

/// Generates a random password from the given character set
///
/// # Arguments
/// * `length` - Desired length of the password
/// * `charset` - String containing all characters to choose from
///
/// # Returns
/// * `String` - Randomly generated password
///
/// # Algorithm
/// For each character position:
/// 1. Shuffle the character set for added randomness
/// 2. Randomly select one character from the shuffled set
/// 3. Append it to the password
fn compute_password(length: u32, charset: &String) -> String {
    // Initialize the random number generator
    let mut rng = rand::rng();

    // Initialize empty password string
    let mut password = String::from("");

    // Convert charset to a vector of characters for shuffling
    let mut charset = charset.chars().collect::<Vec<char>>();

    // Generate each character of the password
    for _ in 0..length {
        // Shuffle the charset for better randomness
        charset.shuffle(&mut rng);

        // Randomly choose one character and append to password
        password.push(charset.choose(&mut rng).unwrap().clone());
    }

    password
}

/// Calculates the estimated time (in seconds) to crack the password
///
/// # Arguments
/// * `charset_length` - Number of unique characters in the character set
/// * `pwd_length` - Length of the generated password
///
/// # Returns
/// * `u128` - Estimated time to crack in seconds
///
/// # Algorithm
/// Calculates the total number of possible passwords by summing:
/// - charset_length^1 (1-character passwords)
/// - charset_length^2 (2-character passwords)
/// - ...
/// - charset_length^pwd_length (full-length passwords)
///
/// Then divides by the assumed cracking speed (bcrypt attempts/second).
///
/// # Note
/// This is a theoretical estimate based on brute-force attacks against
/// bcrypt hashes. Real-world cracking time may vary significantly.
fn compute_time_to_crack(charset_length: u32, pwd_length: u32) -> u128 {
    // Initialize total number of possible passwords
    let mut nt: u128 = 0;

    // Convert to u128 for large number calculations
    let charset_length = charset_length as u128;
    let pwd_length = pwd_length as u128;

    // Sum all possible password combinations from length 1 to pwd_length
    for i in 1..=pwd_length {
        nt += charset_length.pow(i as u32);
    }

    // Divide by cracking speed to get time in seconds
    nt / BCRYPT_CRACKING_SPEED
}

