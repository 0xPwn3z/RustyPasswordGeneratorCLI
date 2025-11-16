// ============================================================================
// Constants
// ============================================================================

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