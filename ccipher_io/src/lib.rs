//! Input/output handling module for the Caesar Cipher application.
//!
//! This module provides functionality for reading text from files or standard input,
//! and writing processed text to files or standard output. It abstracts the I/O operations
//! to support both file-based and stream-based input/output methods.
//!
//! # Features
//!
//! * File input/output support
//! * Standard input/output (stdin/stdout) support
//! * Error handling for I/O operations
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// Reads input text from either a file or standard input.
///
/// # Returns
///
/// * `io::Result<String>` - The content read from the input source, or an IO error.
pub fn read_input(input_file: &Option<PathBuf>) -> io::Result<String> {
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

/// Writes content to either a file or standard output.
///
/// # Returns
///
/// * `io::Result<()>` - `Ok(())` if the write operation succeeds, or an IO error.
pub fn write_output(output_file: &Option<PathBuf>, content: &str) -> io::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;
    use testdir::testdir;

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
