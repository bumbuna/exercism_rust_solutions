use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let s: String = candidate
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect();
    let mut h: HashSet<char> = HashSet::new();
    s.chars().for_each(|c| {
        h.insert(c);
    });
    h.len() == s.len()
}
