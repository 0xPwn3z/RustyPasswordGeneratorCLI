use rand::prelude::{IndexedRandom, SliceRandom};
use crate::utils;
use crate::cli::Args;

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
    if args.length < utils::MIN_LENGTH || args.length > utils::MAX_LENGTH {
        println!("Password length must be between {} and {}. Using default: {}", utils::MIN_LENGTH, utils::MAX_LENGTH, utils::DEFAULT_LENGTH);
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
fn create_charset(args: &Args) -> String {
    // Start with lowercase letters as the base character set
    let mut charset = String::from(utils::CHARS);

    // Add uppercase letters if requested
    if args.uppercase_chars {
        charset.push_str(utils::UPPERCASE_CHARS);
    }

    // Add special characters if requested
    if args.special_chars {
        charset.push_str(utils::SPECIAL_CHARS);
    }

    // Add numbers if requested
    if args.numbers {
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
pub fn compute_password(args: &Args) -> String {
    // Initialize the random number generator
    let mut rng = rand::rng();

    // Initialize empty password string
    let mut password = String::from("");

    // Set the password length and character set
    let length = set_length(args);
    let charset = create_charset(args);

    // Convert charset to a vector of characters for shuffling
    let mut charset = charset.chars().collect::<Vec<char>>();

    // Generate each character of the password
    for _ in 0..length {
        // Shuffle the charset for better randomness
        charset.shuffle(&mut rng);

        // Randomly choose one character and append to password
        password.push(*charset.choose(&mut rng).expect("Charset vuoto"));
    }

    password
}

/*
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
}*/