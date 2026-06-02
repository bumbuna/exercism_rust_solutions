use std::collections::{HashMap, HashSet};


pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let words: Vec<String> = input_to_words(input);
    let letters = unique_letters_in_input(input);
    if letters.len() > 10 {
        return None;
    }
    let leading_letters = first_letters_from_words(&words);
    let mut l2d = HashMap::new();
    let mut used_digits = HashMap::<u8, bool>::new();
    for i in 0..10 {
        used_digits.insert(i, false);
    }

    if let Some(_) = backtrack(0, &letters, &mut l2d, &mut used_digits, &leading_letters, &words) {
         Some(l2d)
    } else {
        None
    }
}

fn input_to_words(input: &str) -> Vec<String> {
    input.split(['+', '=']).filter(
        |s| !s.is_empty()
    ).map(
        |s| String::from(s.trim())
    ).collect::<Vec<String>>()
}

fn unique_letters_in_input(input: &str) -> Vec<char> {
    input.chars().filter(
        |s| s.is_alphabetic()
    ).collect::<HashSet<char>>()
    .into_iter().collect()
}

fn first_letters_from_words(words: &Vec<String>) -> String {
    words.iter().filter_map(
        |w| w.chars().next()
    ).collect()
}

fn backtrack<'a>(index: usize, letters: &'a Vec<char>, l2d: &'a mut HashMap<char, u8>, used: &'a mut HashMap<u8, bool>, leading_letters: &'a str, input: &'a Vec<String>) -> Option<&'a HashMap<char, u8>> {
    if index == letters.len() {
        return if is_valid(input, l2d) {
            Some(l2d)
        } else {
            None
        }
    }

    let current_letter = letters[index];
    for i in 0..=9 {
        if !used.get(&i).unwrap() {
            if i == 0 && leading_letters.contains(current_letter) {
                continue;
            }
            l2d.insert(current_letter, i);
            used.insert(i, true);

            if let Some(_) = backtrack(index+1, letters, l2d, used, leading_letters, input) {
                return Some(l2d);
            }
            used.insert(i, false);
            l2d.remove(&current_letter);
        }
    }
    None
}

fn is_valid(input: &Vec<String>, map: &HashMap<char, u8>) -> bool {
    let nums = input.into_iter().map(|w| {
        let mut sum = 0;
        for c in w.chars() {
            sum *= 10;
            if let Some(v) = map.get(&c) {
                sum += *v as u32;
            }
        }
        sum
    }).collect::<Vec<u32>>();
    let mut running_sum = 0;
    for i in 0..nums.len()-1 {
        running_sum += nums[i];
    }
    running_sum == nums[nums.len()-1]
}
