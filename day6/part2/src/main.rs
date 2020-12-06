use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

const SKIP: &str = "skip";

fn return_last_state(i: usize, length: usize, state: &str) -> Option<String> {
    if i == length - 1 {
        Some(state.to_string())
    } else {
        Some(SKIP.to_string())
    }
}

fn get_common_answers_in_common(s: &str) -> usize {
    let answers_per_person = s.split_whitespace();
    let number_of_persons = answers_per_person.clone().count();
    if number_of_persons == 1 {
        s.len()
    } else {
        let mut set: HashMap<char, usize> = HashMap::new();
        for string in answers_per_person {
            for c in string.chars() {
                *set.entry(c).or_insert(0) += 1;
            }
        }
        dbg!(s, &set);
        dbg!(set
            .iter()
            .filter(|(_, &value)| value == number_of_persons)
            .count())
    }
}

fn main() {
    let stdin = io::stdin();
    let vec: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    let count = vec
        .iter()
        .enumerate()
        .scan(String::new(), |state, (i, val)| {
            let num_fields = val.split_whitespace().count();
            if num_fields == 0 {
                let res = state.clone();
                *state = "".to_string();
                Some(res)
            } else if state.is_empty() {
                *state = val.to_string();
                return_last_state(i, vec.len(), &state)
            } else {
                let line = format!("{} {}", *state, val);
                *state = line;
                return_last_state(i, vec.len(), &state)
            }
        })
        .filter(|x| x != SKIP)
        .fold(0, |acc, string| acc + get_common_answers_in_common(&string));

    println!(
        "Number of questions answered in common in all groups: {}",
        count
    );
}
