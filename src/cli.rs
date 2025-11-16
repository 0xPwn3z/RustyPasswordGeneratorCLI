use clap::{Parser, Subcommand};
use crate::utils::DEFAULT_LENGTH;

// ============================================================================
// Command-Line Arguments
// ============================================================================

/// Command-line arguments structure for password generation configuration
///
/// Uses the `clap` crate to parse command-line arguments with sensible defaults.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// subcommand to execute
    #[command(subcommand)]
    pub(crate) command: Commands,
}

// SUBOCOMMANDS ENUM
#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Generate a new random password
    Generate(GenerateArgs),

    /// Analyze the strength of an existing password
    Analyze(AnalyzeArgs),
}

#[derive(Parser, Debug)]
pub(crate) struct GenerateArgs {
    /// Password length (must be between 8 and 128 characters)
    #[arg(short, long, default_value_t = DEFAULT_LENGTH)]
    pub(crate) length: u32,

    /// Include uppercase characters (A-Z) in the password
    #[arg(short, long, default_value_t = false)]
    pub(crate) uppercase_chars: bool,

    /// Include special characters (!@#$%^&*_-+=<>?) in the password
    #[arg(short, long, default_value_t = false)]
    pub(crate) special_chars: bool,

    /// Include numeric digits (0-9) in the password
    #[arg(short, long, default_value_t = false)]
    pub(crate) numbers: bool,
}

#[derive(Parser, Debug)]
pub(crate) struct AnalyzeArgs {
    /// password to analyze
    #[arg(required = true)]
    pub(crate) password: String,
}