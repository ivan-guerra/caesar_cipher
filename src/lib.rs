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
        CaesarCipher {
            shift: shift.rem_euclid(95), // Ensure shift is within ASCII printable range
        }
    }

    pub fn apply_cipher(&self, text: &str) -> String {
        text.chars()
            .map(|c| self.shift_char(c, self.shift))
            .collect()
    }

    fn shift_char(&self, c: char, shift: i32) -> char {
        if !(' '..='~').contains(&c) {
            return c;
        }

        let base = ' ' as i32;
        let pos = c as i32 - base;
        let shifted = (pos + shift).rem_euclid(95) + base;

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
