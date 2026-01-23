static M1: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five", 
        "six", "seven", "eight", "nine", "ten", 
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", 
        "sixteen", "seventeen", "eighteen", "nineteen"
    ];
static M2: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

static M3: [&str; 7] = [
        "", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion",
    ];

fn n2s(num: usize) -> String {
    if num > 99 {
        let mut s1 = format!("{} hundred", M1[num/100]);
        if num%100 != 0 {
            s1 = format!("{s1} {}", n2s(num%100));
        }
        s1
    } else if num > 19 {
        let mut s1 = M2[num/10].to_string();
        if num%10 != 0 {
            s1 = format!("{s1}-{}", n2s(num%10));
        }
        s1
    }  else {
        M1[num].to_string()
    }
}

pub fn encode(n: u64) -> String {
    
    if n == 0 {
        return "zero".into();
    }
    let mut t = vec!();
    let mut n = n;
    let mut s = String::new();
    while n > 0 {
        t.push(n%1000);
        n /= 1000;
    }
    for (i,v) in t.iter().enumerate() {
        if *v > 0 {
            s = format!("{} {} {s}", n2s(*v as usize), M3[i]);
        }
    }

    s.trim().to_string()
}
