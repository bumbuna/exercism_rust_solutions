fn num_to_word(n: u32) -> &'static str {
    match n {
        10 => "Ten",
        9 => "Nine",
        8 => "Eight",
        7 => "Seven",
        6 => "Six",
        5 => "Five",
        4 => "Four",
        3 => "Three",
        2 => "Two",
        0 => "No",
        _ => "One",
    }
}

fn single_or_plural(count: u32) -> &'static str {
    if count == 1 {
        "bottle"
    } else {
        "bottles"
    }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    let mut start_bottles = start_bottles;
    for _ in 1..=take_down {
        song += format!(
            "{} green {} hanging on the wall,\n\
            {} green {} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {} green {} hanging on the wall.\n\n",
            num_to_word(start_bottles),
            single_or_plural(start_bottles),
            num_to_word(start_bottles),
            single_or_plural(start_bottles),
            num_to_word(start_bottles - 1).to_lowercase(),
            single_or_plural(start_bottles - 1)
        )
        .as_str();
        start_bottles -= 1;
    }
    song
}
