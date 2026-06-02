pub fn translate(input: &str) -> String {
    let mut res = String::new();
    for s in input.split(" ") {
        res = format!("{res} {}", 
            if let Some(s) = rule_1(s) {
                s
            } else if let Some(s) = rule_4(s) {
                s
            } else if let Some(s) = rule_3(s) {
                s
            } else {
                rule_2(s)
            }
        );
    }
    res.trim().into()
}

fn rule_1(s: &str) -> Option<String> {
    if s.starts_with(is_vowel) 
        || s.starts_with("xr") 
        || s.starts_with("yt") {
        Some(format!("{s}ay"))
    } else {
        None
    }
}

fn rule_2(s: &str) -> String {
    let mut index = 0;
    for (i, c) in s.char_indices() {
        if is_vowel(c) {
            index = i;
            break;
        }
    }
    format!("{}{}ay", &s[index..], &s[0..index])
}

fn rule_3(s: &str) -> Option<String> {
    for (i, c)in s.char_indices() {
        if c == 'q' && &s[i..i+2] == "qu"{
            return Some(format!("{}{}quay", &s[i+2..], &s[..i]));
        }
        if is_vowel(c) { 
           break; 
        }
    }
    None
}

fn rule_4(s: &str) -> Option<String> {
    for (i, c)in s.char_indices() {
        if c == 'y' && i != 0{
            return Some(format!("{}{}ay", &s[i..], &s[0..i]));
        }
        if is_vowel(c) { 
           break; 
        }
    }
    None
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a'|'e'|'i'|'o'|'u')    
}
