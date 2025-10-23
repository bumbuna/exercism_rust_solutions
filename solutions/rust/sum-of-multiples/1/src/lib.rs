pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut m = vec![0];
    factors.iter().for_each( |f| {
        let mut multiplier = 1;
        loop {
            if f * multiplier >= limit { break;}
            m.push(f*multiplier);
            multiplier += 1;
        }
    });
    m.sort();
    m.dedup();
    m.iter().sum()
}
