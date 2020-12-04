use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let vec: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .collect();
    let count = vec.iter().filter(|&x| {
        let mut iter = x.split_whitespace();
        let range: Vec<_> = iter.next().unwrap().split("-").filter_map(|x| x.parse::<usize>().ok()).collect();
        let char_to_find = iter.next().unwrap().chars().nth(0).unwrap();
        let string = iter.next().unwrap();
        if string.matches(char_to_find).count() >= *range.get(0).unwrap() && string.matches(char_to_find).count() <= *range.get(1).unwrap() {
            true
        } else {
            false
        }
    }).count();
    println!("Number of valid passwords: {}", count);
}
