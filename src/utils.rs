// ============================================================================
// Constants
// ============================================================================

use colored::Colorize;
use figlet_rs::FIGfont;

/// Default password length when not specified by the user
pub const DEFAULT_LENGTH: u32 = 16;

/// Minimum allowed password length
pub const MIN_LENGTH: u32 = 8;

/// Maximum allowed password length
pub const MAX_LENGTH: u32 = 128;

/// Lowercase alphabet characters
pub const CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

/// Uppercase alphabet characters
pub const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Numeric digits
pub const NUMBERS: &str = "0123456789";

/// Special characters for enhanced password security
pub const SPECIAL_CHARS: &str = "!@#$%^&*_-+=<>?";

/// Assumed bcrypt cracking speed in attempts per second (9,000 attempts/sec)
/// This is a conservative estimate for modern hardware
pub const BCRYPT_CRACKING_SPEED: u128 = 9000;

// ============================================================================
// Helper Functions
// ============================================================================

/// Prints the application logo using ASCII art
///
/// Displays "Rusty Password Generator" in green using FIGfont.
/// Falls back to simple text if FIGfont conversion fails.
pub fn print_logo() {
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

pub(crate) fn analyze_password(p0: &String) -> String {
    todo!()
}