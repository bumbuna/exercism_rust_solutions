pub fn get_diamond(c: char) -> Vec<String> {
    let max_s = add_spaces_btwn(c);
    let mut v = vec!();
    for c_c in 'A'..c {
        let s = add_spaces_btwn(c_c);
        let ts = String::from(" ").repeat((max_s.len()-s.len())/2);
        let mut row = String::from(&ts);
        row.push_str(&s);
        row.push_str(&ts);
        v.push(row);
    }
    let mut v2 = v.to_vec();
    v.push(max_s);
    v2.reverse();
    for s in v2.into_iter() {
        v.push(s);
    }
    v
}

// v 0, 1, 3, 5, 7, 9
// i 0, 1, 2, 3, 4, 5

fn add_spaces_btwn(c: char) -> String {
    let s = (c as usize) - ('A' as usize);
    if s == 0 {
        String::from("A")
    } else {
        let mut s1 = String::new();
        s1.push(c);
        let spaces = String::from(" ").repeat(s*2-1);
        s1.push_str(&spaces);
        s1.push(c);
        s1
    }
}