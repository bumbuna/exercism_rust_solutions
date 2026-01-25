pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut v = Vec::new();
    digits.chars().collect::<Vec<char>>().windows(len).for_each( |w| {
        v.push(w.iter().collect());
    });
    v
}
