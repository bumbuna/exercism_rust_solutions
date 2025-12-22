use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let s: String = candidate
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_alphabetic())
        .collect();
    let mut h: HashSet<char> = HashSet::new();
    s.chars().for_each(|c| {
        h.insert(c);
    });
    h.len() == s.len()
}
