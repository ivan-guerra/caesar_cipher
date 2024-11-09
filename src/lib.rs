use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub struct Config {
    pub input_file: Option<std::path::PathBuf>,
    pub output_file: Option<std::path::PathBuf>,
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

pub struct CaesarCipher {
    shift: i32,
}

impl CaesarCipher {
    pub fn new(shift: i32) -> Self {
        CaesarCipher { shift }
    }

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

fn read_input(input_file: &Option<PathBuf>) -> io::Result<String> {
    match input_file {
        Some(path) => {
            let mut file = File::open(path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        }
        None => {
            let mut content = String::new();
            io::stdin().read_to_string(&mut content)?;
            Ok(content)
        }
    }
}

fn write_output(output_file: &Option<PathBuf>, content: &str) -> io::Result<()> {
    match output_file {
        Some(path) => {
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        }
        None => {
            io::stdout().write_all(content.as_bytes())?;
            Ok(())
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input(&config.input_file)?;
    let output = config.cipher.apply_cipher(&input);
    write_output(&config.output_file, &output)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;
    use testdir::testdir;

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

    #[test]
    fn read_input_from_existing_file_returns_ok() -> io::Result<()> {
        let dir = testdir!();
        let input_path = dir.join("input.txt");
        let content = "Hello\n123\n!@#";
        fs::write(&input_path, content)?;

        let result = read_input(&Some(input_path))?;
        assert_eq!(result, content);
        Ok(())
    }

    #[test]
    fn read_input_from_nonexisting_file_returns_error() {
        let dir = testdir!();
        let nonexistent = dir.join("nonexistent.txt");

        let result = read_input(&Some(nonexistent));
        assert!(result.is_err());
    }

    #[test]
    fn write_output_to_file_returns_ok() -> io::Result<()> {
        let dir = testdir!();
        let output_path = dir.join("output.txt");
        let content = "Hello\x01\x02\x03!@#";

        write_output(&Some(output_path.clone()), content)?;

        let written_content = fs::read_to_string(output_path)?;
        assert_eq!(written_content, content);
        Ok(())
    }

    #[test]
    fn write_output_to_invalid_path_returns_error() {
        let invalid_path = PathBuf::from("/nonexistent/directory/file.txt");
        let content = "Test content";

        let result = write_output(&Some(invalid_path), content);
        assert!(result.is_err());
    }
}
