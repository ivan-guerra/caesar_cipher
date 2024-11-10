//! A Caesar cipher implementation for ASCII characters.
//!
//! This module provides functionality to encrypt and decrypt text using the Caesar cipher,
//! which is a substitution cipher that shifts characters by a fixed number of positions
//! in the ASCII character set (0-127).
//!
//! # Examples
//!
//! ```rust
//! use ccipher::CaesarCipher;
//!
//! let text = "Hello, World!";
//! let shift = 3;
//!
//! let encrypt = CaesarCipher::new(shift);
//! let encrypted = encrypt.apply_cipher(text);
//!
//! let decrypt = CaesarCipher::new(-shift);
//! let decrypted = decrypt.apply_cipher(&encrypted);
//!
//! assert_eq!(text, decrypted);
//! ```
//!
//! # Note
//!
//! This implementation:
//! * Works with all ASCII characters (0-127)
//! * Performs wrapping within the ASCII range
//! * Preserves the original character properties
//! * Applies consistent shifting across the entire ASCII range

/// Configuration structure for the Caesar cipher program.
///
/// # Examples
///
/// ```
/// use ccipher::{Config, CaesarCipher};
/// use std::path::PathBuf;
///
/// let config = Config {
///     input_file: Some(PathBuf::from("input.txt")),
///     output_file: Some(PathBuf::from("output.txt")),
///     cipher: CaesarCipher::new(3),
/// };
/// ```
pub struct Config {
    /// Optional input file path. When None, input is read from standard input (stdin).
    pub input_file: Option<std::path::PathBuf>,
    /// Optional output file path. When None, output is written to standard output (stdout).
    pub output_file: Option<std::path::PathBuf>,
    /// Caesar cipher configuration containing the shift value for character transformation.
    pub cipher: CaesarCipher,
}

impl Config {
    pub fn new(
        key: i32,
        input_file: Option<std::path::PathBuf>,
        output_file: Option<std::path::PathBuf>,
    ) -> Self {
        Config {
            input_file,
            output_file,
            cipher: CaesarCipher::new(key),
        }
    }
}

/// A Caesar cipher implementation for ASCII characters.
///
/// # Examples
///
/// ```
/// use ccipher::CaesarCipher;
///
/// let cipher = CaesarCipher { shift: 3 };
/// assert_eq!(cipher.apply_cipher("Hello!"), "Khoor$");
/// ```
pub struct CaesarCipher {
    /// The number of positions to shift characters in the cipher.
    ///
    /// Positive values shift characters forward in the ASCII range (0-127),
    /// while negative values shift characters backward. The shift wraps around
    /// within the ASCII range.
    pub shift: i32,
}

impl CaesarCipher {
    /// Creates a new CaesarCipher instance with the specified shift value.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccipher::CaesarCipher;
    ///
    /// let cipher = CaesarCipher::new(3);
    /// ```
    pub fn new(shift: i32) -> Self {
        CaesarCipher { shift }
    }

    /// Applies the Caesar cipher transformation to the input text.
    ///
    /// Takes a string slice and shifts each character by the configured shift value,
    /// wrapping around within the ASCII range (0-127).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccipher::CaesarCipher;
    ///
    /// let cipher = CaesarCipher::new(3);
    /// assert_eq!(cipher.apply_cipher("ABC"), "DEF");
    /// ```
    pub fn apply_cipher(&self, text: &str) -> String {
        text.chars()
            .map(|c| self.shift_char(c, self.shift))
            .collect()
    }

    fn shift_char(&self, c: char, shift: i32) -> char {
        if !c.is_ascii() {
            return c;
        }

        let ascii_alphabet_len = 128;
        let pos = c as i32;
        let shifted = (pos + shift).rem_euclid(ascii_alphabet_len);

        char::from_u32(shifted as u32).unwrap_or(c)
    }
}

/// Executes the cipher operation based on the provided configuration.
///
/// # Returns
///
/// `Ok(())` on success, or an error if file operations fail.
///
/// # Errors
///
/// This function will return an error if:
/// * The input file cannot be read
/// * The output file cannot be written
pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let input = ccipher_io::read_input(&config.input_file)?;
    let output = config.cipher.apply_cipher(&input);
    ccipher_io::write_output(&config.output_file, &output)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_cipher_returns_correct_text_on_single_shift() {
        let cipher = CaesarCipher::new(1);
        // Each character should shift by 1 in ASCII
        assert_eq!(cipher.apply_cipher("ABC"), "BCD");
        assert_eq!(cipher.apply_cipher("123"), "234");
        assert_eq!(cipher.apply_cipher("xyz"), "yz{");
    }

    #[test]
    fn apply_cipher_returns_correct_text_on_wrapping_shift() {
        let cipher = CaesarCipher::new(5);
        // ASCII 126 (~) wraps to ASCII ETX (0x3).
        assert_eq!(cipher.apply_cipher("~"), "\u{3}");
        // Multiple character wrap-around.
        assert_eq!(cipher.apply_cipher("}~"), "\u{2}\u{3}");
    }

    #[test]
    fn apply_cipher_returns_correct_text_on_zero_shift() {
        let cipher = CaesarCipher::new(0);
        assert_eq!(cipher.apply_cipher("Hello 123 !@#"), "Hello 123 !@#");
    }

    #[test]
    fn apply_cipher_returns_correct_text_on_large_shift() {
        let cipher = CaesarCipher::new(128);
        // 128 is equivalent to 0 for ASCII
        assert_eq!(cipher.apply_cipher("ABC"), "ABC");
    }

    #[test]
    fn apply_cipher_returns_correct_text_on_ctrl_chars() {
        let cipher = CaesarCipher::new(1);
        assert_eq!(cipher.apply_cipher("\x00\x01\x02"), "\x01\x02\x03");
    }

    #[test]
    fn apply_cipher_returns_correct_text_on_empty_str() {
        let cipher = CaesarCipher::new(5);
        assert_eq!(cipher.apply_cipher(""), "");
    }

    #[test]
    fn apply_cipher_returns_correct_text_on_full_ascii_range() {
        let cipher = CaesarCipher::new(1);
        let input: String = (0..=127).map(char::from).collect();
        let expected: String = (1..=127)
            .chain(std::iter::once(0))
            .map(char::from)
            .collect();
        assert_eq!(cipher.apply_cipher(&input), expected);
    }

    #[test]
    fn apply_cipher_returns_correct_text_on_negative_shift() {
        let cipher = CaesarCipher::new(-1);
        assert_eq!(cipher.apply_cipher("ABC"), "@AB");
        assert_eq!(cipher.apply_cipher("\x01"), "\x00");
    }
}
