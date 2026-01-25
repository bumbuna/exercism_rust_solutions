pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u64> = vec![2,3];
    let mut suspect: u64 = 5;
    loop {
        if primes.len() as u32 > n+2 { break; }
        let mut is_prime = true;
        for i in &primes {
            if *i * *i > suspect {break;}
            if suspect % *i == 0 { is_prime = false; break;}
        }    
        if is_prime {
            primes.push(suspect);
        }
        suspect += 2;
    }
    primes[n as usize] as u32
}
