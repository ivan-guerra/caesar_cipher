use clap::ValueEnum;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::path::PathBuf;

const ASCII_ALPHABET_LEN: u8 = 128;
const POPULAR_ENGLISH_WORDS: &str = include_str!("../datasets/popular_english_words.txt");
const FREQUENCY_TABLE: &str = include_str!("../datasets/ascii_char_frequencies.txt");

#[derive(Clone, Debug, ValueEnum)]
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

fn load_dictionary() -> HashSet<String> {
    POPULAR_ENGLISH_WORDS
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}

fn apply_ascii_dict_attack(ciphertext: &str, dictionary: &HashSet<String>) -> Option<u8> {
    // Count the number of dictionary words for each shift
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
        // Return None if all shifts in scores have a value of 0
        None
    } else {
        // Return the shift with highest score
        Some(
            scores
                .iter()
                .max_by_key(|&(_, &count)| count)
                .map(|(&shift, _)| shift)
                .unwrap_or(0),
        )
    }
}

fn get_freq_distribution(char_counter: &BTreeMap<char, u32>) -> Vec<f64> {
    if char_counter.is_empty() {
        return vec![0.0; ASCII_ALPHABET_LEN.into()];
    }

    let total_chars: u32 = char_counter.values().sum();

    (0..ASCII_ALPHABET_LEN)
        .map(|c| {
            let count = char_counter.get(&char::from(c)).unwrap_or(&0);
            f64::from(*count) / f64::from(total_chars)
        })
        .collect()
}

fn apply_ascii_freq_attack(ciphertext: &str) -> u8 {
    type CharCounter = BTreeMap<char, u32>;
    type ShiftCharCounts = BTreeMap<u8, CharCounter>;

    // Count the frequency of each character for each shift
    let mut shift_counts: ShiftCharCounts = BTreeMap::new();
    for shift in 0..ASCII_ALPHABET_LEN {
        let cipher = ccipher::CaesarCipher::new(i32::from(shift));
        let plaintext = cipher.apply_cipher(ciphertext);

        for c in plaintext.chars() {
            if c.is_ascii() {
                let count = shift_counts.entry(shift).or_default();
                *count.entry(c).or_insert(0) += 1;
            }
        }
    }

    // Calculate frequency distribution for each shift
    let freq_distributions: Vec<Vec<f64>> =
        shift_counts.values().map(get_freq_distribution).collect();

    // Find the shift with the closest distribution to the reference ASCII frequency table
    let freq_table: Vec<f64> = FREQUENCY_TABLE
        .lines()
        .map(|line| line.parse::<f64>().unwrap())
        .collect();
    let mut min_diff = f64::INFINITY;
    let mut best_shift = 0;
    for (shift, distribution) in freq_distributions.iter().enumerate() {
        let diff = freq_table
            .iter()
            .zip(distribution.iter())
            .map(|(f1, f2)| (f1 - f2).abs())
            .sum();
        if diff < min_diff {
            min_diff = diff;
            best_shift = shift as u8;
        }
    }

    best_shift
}

pub fn run(config: &Config) -> io::Result<()> {
    let ciphertext = ccipher_io::read_input(&config.ciphertext_file)?;
    let shift = match config.attack_type {
        Attack::Dictionary => {
            let dictionary = load_dictionary();
            apply_ascii_dict_attack(&ciphertext, &dictionary)
        }
        Attack::Frequency => Some(apply_ascii_freq_attack(&ciphertext)),
    };

    match shift {
        Some(shift) => {
            println!("candidate key: {}", shift);
        }
        None => println!("unable to find candidate key"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs;

    fn get_test_dictionary_path() -> PathBuf {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        PathBuf::from(manifest_dir)
            .join("datasets")
            .join("popular_english_words.txt")
    }

    #[test]
    fn load_dictionary_succeeds_on_valid_file() {
        let result = load_dictionary();

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
        let result = load_dictionary();

        // Verify that empty lines are filtered out
        assert!(result.len() <= total_lines);
        assert!(!result.contains(""));
        assert!(!result.iter().any(|word| word.trim().is_empty()));
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

        // Given the encryption key is 1, the decryption key is -1 % ASCII_ALPHABET_LEN = 127
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

    #[test]
    fn get_freq_distribution_returns_zeroes_on_empty_char_counter() {
        let char_counter = BTreeMap::new();
        let distribution = get_freq_distribution(&char_counter);

        assert_eq!(distribution.len(), ASCII_ALPHABET_LEN.into());
        assert!(distribution.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn get_freq_distribution_returns_correct_dist_when_char_counter_has_single_char() {
        let mut char_counter = BTreeMap::new();
        char_counter.insert('a', 1);
        let distribution = get_freq_distribution(&char_counter);

        assert_eq!(distribution.len(), ASCII_ALPHABET_LEN.into());
        assert_eq!(distribution[97], 1.0); // 'a' is ASCII 97
        assert_eq!(distribution.iter().sum::<f64>(), 1.0);
    }

    #[test]
    fn get_freq_distribution_returns_correct_dist_when_char_counter_has_multiple_chars() {
        let mut char_counter = BTreeMap::new();
        char_counter.insert('a', 2);
        char_counter.insert('b', 1);
        char_counter.insert('c', 1);
        let distribution = get_freq_distribution(&char_counter);

        assert_eq!(distribution.len(), ASCII_ALPHABET_LEN.into());
        assert_eq!(distribution[97], 0.5); // 'a' frequency
        assert_eq!(distribution[98], 0.25); // 'b' frequency
        assert_eq!(distribution[99], 0.25); // 'c' frequency
        assert_eq!(distribution.iter().sum::<f64>(), 1.0);
    }

    #[test]
    fn apply_ascii_freq_attack_returns_key_when_given_basic_text() {
        let ciphertext =
            "The ancient manuscript revealed a forgotten story about a small village in \
    the mountains. Every winter, when the snow reached the windowsills, the villagers would \
    gather in the town hall to share tales and warm soup. They had a peculiar tradition of \
    writing their hopes for spring on paper lanterns, which they would release into the night \
    sky on the longest evening of winter. Year after year, this ritual brought the community \
    together, creating bonds that lasted generations.";
        let shift = 3;
        let encrypted = ccipher::CaesarCipher::new(shift).apply_cipher(ciphertext);
        let detected_shift = -i32::from(apply_ascii_freq_attack(&encrypted));

        assert_eq!(detected_shift.rem_euclid(ASCII_ALPHABET_LEN.into()), shift);
    }

    #[test]
    fn apply_ascii_freq_attack_returns_zero_on_empty_ciphertext() {
        let ciphertext = "";
        let detected_shift = apply_ascii_freq_attack(ciphertext);

        assert_eq!(detected_shift, 0);
    }

    #[test]
    fn apply_ascii_freq_attack_returns_key_on_non_ascii_text() {
        let ciphertext = "Hello, 世界!";
        let shift = 5;
        let encrypted = ccipher::CaesarCipher::new(shift).apply_cipher(ciphertext);
        let detected_shift = -i32::from(apply_ascii_freq_attack(&encrypted));

        assert_eq!(detected_shift.rem_euclid(ASCII_ALPHABET_LEN.into()), shift);
    }
}
