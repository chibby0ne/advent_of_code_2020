use std::collections::HashSet;
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

fn unique_chars(s: &str) -> usize {
    let set: HashSet<char> = s.chars().filter(|x| x.is_alphabetic()).collect();
    set.len()
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
        .fold(0, |acc, string| acc + unique_chars(&string));

    println!("Number of questions asked per group: {}", count);
}
