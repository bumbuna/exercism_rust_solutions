use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        None
    } else if num == 1 {
        Some(Classification::Deficient)
    } else {
        let mut v = HashSet::new();
        v.insert(1);
        for i in 2..num {
            if i*i > num {
                break;
            } else {
                if num%i == 0 {
                    v.insert(i);
                    v.insert(num/i);
                }
            }
        }
        let sum: u64 = v.iter().sum();
        // println!("---------Aliquot Sum of {num} is:[{:?}] {sum}------", v);
       Some( if sum == num {
            Classification::Perfect
        } else if sum > num {
            Classification::Abundant
        } else {
            Classification::Deficient
        })
    }
}
