/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut s = String::new();
    for c in code.chars() {
        if c.is_whitespace() {
            continue;
        }
        if !c.is_ascii_digit() {
            return false;
        };
        s.push(c);
    }
    if s.len() <= 1 {
        return false;
    };
    let mut luhn_sum: u32 = 0;
    let s_is_even_lengthed = s.len() % 2 == 0;

    for (i, c) in s.char_indices() {
        luhn_sum += _digit_value(
            c,
            if s_is_even_lengthed {
                i % 2 == 0
            } else {
                i % 2 == 1
            },
        );
    }

    luhn_sum % 10 == 0
}

fn _digit_value(c: char, to_double: bool) -> u32 {
    let mut x = c.to_digit(10).unwrap();
    if to_double {
        x += x;
        if x > 9 {
            x -= 9;
        }
    }
    x
}
