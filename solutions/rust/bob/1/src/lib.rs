fn is_yell_question(s: &str) -> (bool, bool) {
    let mut yell = true;
    let mut question = false;
    let m = &s[0..s.len()-1];
    let last_c = s[s.len()-1..].chars().next().unwrap();
    let mut has_letter = false;

    for c in m.chars() {
        if c.is_ascii_digit() || c.is_ascii_punctuation() || c.is_ascii_whitespace() {
            continue;
        }
        has_letter = true;
        yell = yell && c.is_ascii_uppercase();
    }

    if last_c == '?' {
        question = true;
    } 

    if yell && !question {
        yell = last_c.is_ascii_uppercase() || !last_c.is_ascii_alphabetic();
    }
    
    (yell && has_letter, question)
}

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let (y, q) = is_yell_question(message);
    
    if y && q {
        "Calm down, I know what I'm doing!"
    } else if y {
        "Whoa, chill out!"
    } else if q {
        "Sure."
    } else {
        "Whatever." 
    }
}
