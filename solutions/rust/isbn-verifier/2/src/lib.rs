/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let s: String = isbn.chars().filter(|c| c.is_alphanumeric()).collect();
    let s2: String = s
        .chars()
        .filter(|c| c.is_ascii_digit() || (c.is_alphabetic() && *c == 'X'))
        .collect();
    if s != s2 || s.len() != 10 {
        return false;
    }
    if s.contains('X') && !s.ends_with('X') {
        //X is not at end of isbn
        return false;
    }
    s.chars()
        .map(|c| {
            if c == 'X' {
                10
            } else {
                c as usize - ('0' as usize)
            }
        })
        .enumerate()
        .fold(0, |a, (i, v)| a + v * (10 - i))
        % 11
        == 0
}
