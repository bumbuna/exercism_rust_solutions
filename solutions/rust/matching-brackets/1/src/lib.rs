// #[derive(Clone)]
// struct Item {
//     c: Option<char>,
//     next: Option<Box<Item>>
// } 

// struct Queue {
//     list: Box<Item>
// }

// impl Queue {
//     fn new() -> Self {
//         Self {
//             list: Box::new(Item {
//                 c: None,
//                 next: None
//             })
//         }
//     }

//     fn push(&mut self, c: char) {
//         let new_item = Box::new(Item {
//             c: Some(c),
//             next: Some(self.list.clone())
//         });
//         self.list = new_item;
//     }

//     fn pop(&mut self) -> Option<char> {
//         if let Some(c) = self.list.c {
//             self.list = self.list.clone().next.unwrap();
//             Some(c)
//         } else {
//             None
//         }
//     }

//     fn is_empty(&self) -> bool {
//         self.list.next.is_none()
//     }
// }

fn is_match(a: char, b: char) -> bool {
    a == '{' && b == '}' ||
    a == '[' && b == ']' ||
    a == '(' && b == ')'
}

fn is_open(c: char) -> bool {
    c == '{' || c == '[' || c == '('
}

fn is_close(c: char) -> bool {
    c == '}' || c == ']' || c == ')'
}

// pub fn brackets_are_balanced(string: &str) -> bool {
//     let mut q = Queue::new();
//     for c in string.chars() {
//         if c == '{' || c == '[' || c == '(' {
//             q.push(c);
//         } else if c == '}' || c == ']' || c == ')' {
//             if let Some(qc) = q.pop() {
//                 if !is_match(qc, c) {return false;}
//             } else {
//                 return false;
//             }
//         }
//     }
// q.is_empty()
// }

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut q: Vec<char> = Vec::new();
    for c in string.chars() {
        if is_open(c) {
            q.push(c);
        } else if is_close(c) {
            if let Some(qc) = q.pop() {
                if !is_match(qc, c) {return false;}
            } else {
                return false;
            }
        }
    }
    q.is_empty()
}
