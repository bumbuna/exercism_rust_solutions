use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    // let mut h = HashSet::new();
    sentence
    .chars()
    .filter_map(|c| {
        if c.is_ascii_alphabetic() {
            Some(c.to_ascii_lowercase())
    } else {
            None
    }
    })
    .collect::<HashSet<char>>()
    .len() == 26
    // h.len() == 26
}
