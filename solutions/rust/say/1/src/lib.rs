use std::collections::HashMap;

static m1: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five", 
        "six", "seven", "eight", "nine", "ten", 
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", 
        "sixteen", "seventeen", "eighteen", "nineteen"
    ];
static m2: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

static m3: [&str; 7] = [
        "", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion",
    ];

fn num_to_string(num: usize) -> String {
    let s = if num > 99 {
        format!( "{} hundred {}", m1[num/100], 
                if num%100 != 0 {
                    num_to_string(num%100) 
                } else {
                    "".into()   
                }
        )
    } else if num > 19 {
        format!( "{}{}", m2[num/10],
                if num%10 != 0 {
                    format!("-{}", num_to_string(num%10))   
                } else {
                    "".into()
                }
        )
    }  else {
        format!("{}", m1[num])
    };

    s.trim().to_string()
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
            s = format!("{} {} {s}", num_to_string(*v as usize), m3[i]);
        }
    }

    s.trim().to_string()
}
