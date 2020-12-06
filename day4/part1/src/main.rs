use std::io;
use std::io::prelude::*;

const FIELDS: [&str; 7] = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
const SKIP: &str = "skip";

fn return_last_state(i: usize, length: usize, state: &str) -> Option<String> {
    if i == length - 1 {
        Some(state.to_string())
    } else {
        Some(SKIP.to_string())
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
        .filter(|x| FIELDS.iter().all(|&key| x.contains(key)))
        .count();

    println!("Number of valid passports is: {}", count);
}
