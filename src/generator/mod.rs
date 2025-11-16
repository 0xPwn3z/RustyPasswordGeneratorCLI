use rand::prelude::{IndexedRandom, SliceRandom};
use crate::utils;

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
fn set_length (args_length: u32) -> u32 {
    let mut length = args_length;

    // Check if length is within valid bounds. If not, reset to default.
    if args_length < utils::MIN_LENGTH || args_length > utils::MAX_LENGTH {
        length = utils::DEFAULT_LENGTH;
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
fn create_charset(up_chars: bool, spec_chars: bool, num_chars: bool) -> String {
    // Start with lowercase letters as the base character set
    let mut charset = String::from(utils::CHARS);

    // Add uppercase letters if requested
    if up_chars {
        charset.push_str(utils::UPPERCASE_CHARS);
    }

    // Add special characters if requested
    if spec_chars {
        charset.push_str(utils::SPECIAL_CHARS);
    }

    // Add numbers if requested
    if num_chars {
        charset.push_str(utils::NUMBERS);
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
pub fn compute_password(args_length: u32, up_chars: bool, spec_chars: bool, num_chars: bool) -> String {
    // Initialize the random number generator
    let mut rng = rand::rng();

    // Initialize empty password string
    let mut password_chars : Vec<char> = Vec::new();

    // Set the password length and character set
    let length = set_length(args_length);
    let charset = create_charset(up_chars, spec_chars, num_chars);

    // Convert charset to a vector of characters for shuffling
    let mut charset = charset.chars().collect::<Vec<char>>();

    // Shuffle the charset for better randomness
    charset.shuffle(&mut rng);

    // Ensure at least one character from each selected category is included
    let chars: Vec<char> = utils::CHARS.chars().collect();
    password_chars.push(*chars.choose(&mut rng).expect("No characters available"));

    if up_chars {
        // Ensure at least one uppercase character is included
        let upper_chars: Vec<char> = utils::UPPERCASE_CHARS.chars().collect();
        password_chars.push(*upper_chars.choose(&mut rng).expect("No uppercase characters available"));
    }
    if spec_chars {
        // Ensure at least one special character is included
        let special_chars: Vec<char> = utils::SPECIAL_CHARS.chars().collect();
        password_chars.push(*special_chars.choose(&mut rng).expect("No special characters available"));
    }
    if num_chars {
        // Ensure at least one numeric character is included
        let number_chars: Vec<char> = utils::NUMBERS.chars().collect();
        password_chars.push(*number_chars.choose(&mut rng).expect("No numeric characters available"));
    }

    let sub_lenght = password_chars.len() as u32;

    // Generate each character of the password
    for _ in sub_lenght..length {
        // Randomly choose one character and append to password
        password_chars.push(*charset.choose(&mut rng).expect("Empty character set"));
    }

    // Shuffle the final password characters to avoid predictable patterns
    password_chars.shuffle(&mut rng);

    password_chars.iter().collect()
}