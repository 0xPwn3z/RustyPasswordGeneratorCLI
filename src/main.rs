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

use clap::Parser;
use colored::Colorize;
use figlet_rs::FIGfont;

mod generator;
mod utils;
mod cli;
mod analyzer;

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
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Generate(args) => {
            // Generate the random password
            let password = generator::compute_password(args.length, args.uppercase_chars, args.special_chars, args.numbers);
            // Display the generated password and security information
            println!("Generated Password: {}", password);
        },
        cli::Commands::Analyze(args) => {
            let strength = analyzer::analyze_password(&args.password);
            println!("Password Strength Analysis:\n{}", strength);
        }
    }
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
    let standard_font = FIGfont::standard().expect("Failed to read standard font");

    // Convert "Rusty" to ASCII art
    let figure = standard_font.convert("Rusty");

    // Convert "Password Generator" to ASCII art
    let figure1 = standard_font.convert("Password Utility");

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

