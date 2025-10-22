pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u64> = vec![2,3];
    let mut suspect: u64 = 5;
    loop {
        if primes.len() > (n as usize)+2 { break; }
        let mut d_i = 0;
        while d_i < primes.len() {
            if primes[d_i]*primes[d_i] >= suspect { break; }
            d_i += 1;
        }
        let mut f = false;
        for i in 0..=d_i {
            if suspect%primes[i] == 0 { f = true; break;}
        }
        if !f {
        primes.push(suspect);
        }
        suspect += 2;
    }
    primes[n as usize] as u32
}
