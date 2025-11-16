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
mod generator;
mod utils;
mod cli;


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
    utils::print_logo();
    // Parse command-line arguments
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Generate(args) => {
            // Generate the random password
            let password = generator::compute_password(&args);
            // Display the generated password and security information
            println!("Generated Password: {}", password);
        },
        cli::Commands::Analyze(args) => {
            let strength = utils::analyze_password(&args.password);
            println!("Password Strength Analysis:\n{}", strength);
        }
    }
}

