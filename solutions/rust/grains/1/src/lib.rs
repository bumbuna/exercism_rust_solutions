pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!()
    };
    if s == 1 {
        1
    } else {
        2u64.pow(s - 1)
    }
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=64 {
        sum += square(i);
    }
    sum
}
