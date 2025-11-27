use std::collections::HashMap;

// struct Student {
//     name: String,
//     stream: u32
// }

pub struct School {
    roaster: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        Self {
            roaster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.roaster.contains_key(student) {
            self.roaster.insert(student.to_string(), grade);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut v = self.roaster.values().map(|a| *a).collect::<Vec<u32>>();
        v.sort();
        v.dedup();
        v
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut v: Vec<String> = self
            .roaster
            .iter()
            .filter_map(|(a, b)| if *b == grade { Some(a.clone()) } else { None })
            .collect();
        v.sort();
        v
    }
}
