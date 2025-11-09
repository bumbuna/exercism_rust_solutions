pub fn egg_count(display_value: u32) -> usize {
    let mut count: usize = 0;
    let mut d = display_value;
    while d != 0 {
        if d & 1 == 1 {
            count += 1;
        }
        d = d >> 1;
    }
    count
}
