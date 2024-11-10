use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::PathBuf;

pub enum Attack {
    Dictionary,
    Frequency,
}

pub struct Config {
    pub ciphertext_file: Option<PathBuf>,
    pub attack_type: Attack,
}

impl Config {
    pub fn new(ciphertext_file: Option<PathBuf>, attack_type: Attack) -> Self {
        Config {
            ciphertext_file,
            attack_type,
        }
    }
}

fn load_dictionary(dict_path: &PathBuf) -> io::Result<HashSet<String>> {
    let content = fs::read_to_string(dict_path)?;

    Ok(content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect())
}

fn apply_ascii_dict_attack(ciphertext: &str, dictionary: &HashSet<String>) -> Option<u8> {
    const ASCII_ALPHABET_LEN: u8 = 128;
    let mut scores: HashMap<u8, usize> = HashMap::new();

    for shift in 0..ASCII_ALPHABET_LEN {
        let cipher = ccipher::CaesarCipher::new(shift as i32);
        let plaintext = cipher.apply_cipher(ciphertext);
        scores.insert(
            shift,
            plaintext
                .split_whitespace()
                .filter(|&word| dictionary.contains(word))
                .count(),
        );
    }

    if scores.values().all(|&count| count == 0) {
        // Return None if all shifts in scores have a value of 0.
        None
    } else {
        // Return the shift with highest score.
        Some(
            scores
                .iter()
                .max_by_key(|&(_, &count)| count)
                .map(|(&shift, _)| shift)
                .unwrap_or(0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs;

    fn get_test_dictionary_path() -> PathBuf {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        PathBuf::from(manifest_dir)
            .join("dictionaries")
            .join("popular.txt")
    }

    #[test]
    fn load_dictionary_succeeds_on_valid_file() {
        let dict_path = get_test_dictionary_path();
        let result = load_dictionary(&dict_path).unwrap();

        // Assuming the dictionary is not empty
        assert!(!result.is_empty());
        // Assuming some common words exist in the dictionary
        assert!(result.contains("the"));
        assert!(result.contains("and"));
    }

    #[test]
    fn load_dictionary_filters_empty_lines() {
        let dict_path = get_test_dictionary_path();
        let content = fs::read_to_string(&dict_path).unwrap();
        let total_lines = content.lines().count();
        let result = load_dictionary(&dict_path).unwrap();

        // Verify that empty lines are filtered out
        assert!(result.len() <= total_lines);
        assert!(!result.contains(""));
        assert!(!result.iter().any(|word| word.trim().is_empty()));
    }

    #[test]
    fn load_dictionary_returns_error_on_nonexistent_file() {
        let nonexistent = PathBuf::from("dictionaries").join("nonexistent_file.txt");
        let result = load_dictionary(&nonexistent);

        assert!(result.is_err());
    }

    fn create_test_dictionary() -> HashSet<String> {
        vec![
            "the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog", "hello", "world",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn apply_ascii_dict_attack_returns_correct_key_on_valid_text() {
        let dictionary = create_test_dictionary();
        // "hello world" shifted by 1
        let ciphertext = "ifmmp!xpsme";
        let shift = apply_ascii_dict_attack(ciphertext, &dictionary);

        // Given the encryption key is 1, the decryption key is -1 % 128 = 127
        assert_eq!(shift, Some(127));
        // Verify decryption works
        let cipher = ccipher::CaesarCipher::new(shift.unwrap() as i32);
        assert_eq!(cipher.apply_cipher(ciphertext), "hello world");
    }

    #[test]
    fn apply_ascii_dict_attack_returns_none_when_no_matching_words() {
        let dictionary = create_test_dictionary();
        // Random text that won't match any dictionary words
        let ciphertext = "xyz123 abc456";
        let shift = apply_ascii_dict_attack(ciphertext, &dictionary);

        // Should return None when no matches found
        assert_eq!(shift, None);
    }

    #[test]
    fn apply_ascii_dict_attack_returns_a_correct_key_on_multiple_matches() {
        let dictionary = create_test_dictionary();
        // "the fox" shifted by 3
        let ciphertext = "wkh#ir{";
        let shift = apply_ascii_dict_attack(ciphertext, &dictionary);

        assert_eq!(shift, Some(125));
        let cipher = ccipher::CaesarCipher::new(shift.unwrap() as i32);
        assert_eq!(cipher.apply_cipher(ciphertext), "the fox");
    }

    #[test]
    fn apply_ascii_dict_attack_returns_none_on_empty_str_ciphertext() {
        let dictionary = create_test_dictionary();
        let ciphertext = "";
        let shift = apply_ascii_dict_attack(ciphertext, &dictionary);

        assert_eq!(shift, None);
    }
}
