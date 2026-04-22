use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut m = HashMap::new();
    let sep = [':', '?', ':', '\t', '\n', ' ', '.', '!', ',', '&'];
    for w in words.split(|c:char| 
                        c.is_whitespace() || (!c.is_alphanumeric() && c != '\'')).filter(|wd| !wd.is_empty()) {
        let mut s = String::from(w);
        if s.starts_with('\'') {
            let _ = s.remove(0);
        }
        if s.ends_with('\'') {
            let _ = s.pop();
        }
        if s.is_empty() { continue; }
        m.entry(s.to_lowercase()).and_modify(|c| *c += 1).or_insert(1);
    }
    m
}
