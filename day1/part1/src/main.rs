use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let map: HashMap<i64, bool> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .into_iter()
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .into_iter()
        .map(|x| (x, true))
        .collect();
    for (first_factor, _) in &map {
        let second_factor = 2020 - first_factor;
        if let Some(_) = map.get(&second_factor) {
            println!("Pairs are: {} and {}, and product of them is: {}", first_factor, second_factor, first_factor * second_factor);
            break;
        }
    }
}
