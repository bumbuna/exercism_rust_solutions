pub fn number(user_number: &str) -> Option<String> {
    let mut i1 = user_number.chars();
    if user_number.starts_with("+") {
        i1.next();
        if i1.next() != Some('1') {
            return None;
        }
    }
    let mut s: String = i1.filter(|c| c.is_ascii_digit()).collect();
    s = if s.len() == 11 {
        if s.starts_with('1') {
            let mut i1 = s.chars();
            i1.next();
            i1.collect()
        } else {
            String::from("")
        }
    } else {
        s
    };
    if s.len() != 10 {
        return None;
    }
    let v: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
    if v[0] < 2 || v[3] < 2 {
        return None;
    }

    Some(s)
}
