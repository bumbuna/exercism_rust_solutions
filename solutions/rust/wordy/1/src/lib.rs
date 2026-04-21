#[derive(Debug)]
enum Op {
    Add, Sub, Div, Mul
}

#[derive(Debug)]
enum Token {
    Operator(Op),
    Operand(i32),
    Eoe
}

fn tokenizer(s: &str) -> Option<Vec<Token>> {
    let mut v = vec!();
    let mut s = String::from(s);
    if Some('?') != s.pop() {
        return None;
    }
    let mut i1 = s.split(" ");
    if i1.next() != Some("What") || i1.next() != Some("is") {
        return None;
    }
    while let Some(w) = i1.next() {
        match w {
            "plus" => v.push(Token::Operator(Op::Add)),
            "minus" => v.push(Token::Operator(Op::Sub)),
            "multiplied" => v.push(Token::Operator(Op::Mul)),
            "divided" => v.push(Token::Operator(Op::Div)),
            "by" => continue,
            _ => {
                if let Ok(n) = w.parse::<i32>() {
                    v.push(Token::Operand(n));
                } else {
                    return None;
                }
            }
        }
    }
    v.push(Token::Eoe);
    Some(v)
}

pub fn answer(command: &str) -> Option<i32> {
    let mut i1 = tokenizer(command)?.into_iter();
    let mut result;
    if let Some(t) = i1.next() && let Token::Operand(d) = t {
        result = d;
        while let Some(t) = i1.next() && let Token::Operator(o) = t {
            let mut r1;
            if let Some(tkn) = i1.next() && let Token::Operand(num) = tkn {
                r1 = num;
            } else {
                return None;
            }
            match o {
                Op::Add => result += r1,
                Op::Sub => result -= r1,
                Op::Mul => result *= r1,
                Op::Div => result /= r1,
            }
        }
    } else {
        return None;
    } 
    if i1.next().is_none() {
        Some(result)
    } else {
        None
    }
    // todo!("Return the result of the command '{command}' or None, if the command is invalid.");
}
