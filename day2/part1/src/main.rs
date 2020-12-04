use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let vec: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();
    let count = vec
        .iter()
        .filter(|&x| {
            // line looks like this:
            // 1-3 a: abcde
            let mut iter = x.split_whitespace();
            let mut range_iter = iter
                .next()
                .unwrap()
                .split('-')
                .filter_map(|x| x.parse::<usize>().ok());
            let (min, max) = (range_iter.next().unwrap(), range_iter.next().unwrap());
            let char_to_find = iter.next().unwrap().chars().next().unwrap();
            let count = iter.next().unwrap().matches(char_to_find).count();
            count >= min && count <= max
        })
        .count();
    println!("Number of valid passwords: {}", count);
}
