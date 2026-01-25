fn get_next_prime(prime_store: &mut Vec<u64>, last_prime: u64) -> u64 {
    if last_prime <= 2 {
        return last_prime + 1;
    }
    let mut suspect = last_prime + 2;
    loop {
        let mut is_prime = true;
        for v in &*prime_store {
            if *v * *v > suspect { break; }
            if suspect % *v == 0 {is_prime = false; break; }
        }
        if is_prime {
            prime_store.push(suspect);
            break;
        }
        suspect += 2;
    }

    suspect
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut pf = vec![];
    let mut n =n;
    let mut prime_store: Vec<u64> = vec![2,3];
    let mut last_prime = 1;
    loop {
        loop {
            if last_prime == 1 {break;}
            if n%last_prime != 0 { break; }
            n /= last_prime;
            pf.push(last_prime);
        }
        if n == 1 {break;}
        last_prime = get_next_prime(&mut prime_store, last_prime);
    }
    pf
}

#[cfg(test)]
#[test]
fn first_prime() {
    let mut v: Vec<u64> = vec![2, 3];
    assert_eq!(get_next_prime(&mut v, 1), 2);
}

#[test]
fn second_prime() {
    let mut v: Vec<u64> = vec![2, 3];
    assert_eq!(get_next_prime(&mut v, 2), 3);
}

#[test]
fn fourth_prime() {
    let mut v: Vec<u64> = vec![2, 3, 5];
    assert_eq!(get_next_prime(&mut v, 5), 7);
}
