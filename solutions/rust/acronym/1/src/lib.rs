pub fn abbreviate(phrase: &str) -> String {
    phrase.replace("_", " ")
    .split(|c: char|{
        c == ' ' || c == '-'
    })
    .filter(|s|{
        s.len() > 0
    })
    .map(|s| {
        let mut s1 = String::new();
        let mut chars = s.chars();
        s1.push(chars.next().unwrap().to_uppercase().next().unwrap());
        if !is_capitalized(s) {
            chars.for_each(|c| {
                if c.is_uppercase() {
                    s1.push(c);
                }
            });
        }
        s1
    })
    .collect::<String>()
}

fn is_capitalized(s: &str) -> bool {
    for c in s.chars() {
        if c.is_lowercase() {
            return false;
        }
    }
    true
}
