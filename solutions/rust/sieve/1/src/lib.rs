pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut v = vec!();
    for i in 1..=upper_bound {
        v.push(i);
    }
    v[0] = 0;
    for i in 1..upper_bound {
        if v[i as usize] == 0 { continue; }
        let mut p = v[i as usize] * 2;
        while p <= upper_bound {
            v[p as usize-1] = 0;
            p += v[i as usize]; 
        }
    }
    v.into_iter().filter(|i| *i != 0).collect()
}
