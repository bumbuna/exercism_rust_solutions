pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {return None}
    let mut steps = 0;
    let mut n = n;
    loop {
        if n == 1 {break;}
         n = if n%2 == 0 {
             n/2
         } else {
             n*3+1
         };
        steps += 1;
    }
    Some(steps)
}
