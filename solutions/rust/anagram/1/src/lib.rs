use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut h = HashSet::new();
    let word = String::from(word).to_lowercase();
    let mut word_sorted = word.chars().collect::<Vec<char>>();
    word_sorted.sort();
    let word_sorted: String = word_sorted.into_iter().collect();
    for p in possible_anagrams {
        let s = String::from(*p).to_lowercase();
        if word == s {
            continue;
        }
        let mut s = s.chars().collect::<Vec<char>>();
        s.sort();
        let s: String = s.iter().into_iter().collect();

        if s == word_sorted {
            h.insert(*p);
        }
    }

    h
}
