/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars().map(|c| {
        if c.is_ascii() {
        match c.to_uppercase().next().unwrap() {
            'A'|'E'|'I'|'O'|'U'|'L'|'N'|'R'|'S'|'T' => 1,
            'D'|'G' => 2,
            'B'|'C'|'M'|'P' => 3,
            'F'|'H'|'V'|'W'|'Y' => 4,
            'K' => 5,
            'J'|'X' => 8,
            _ => 10,
            
    }
        } else {
            0
        }
                     }).sum()
}
