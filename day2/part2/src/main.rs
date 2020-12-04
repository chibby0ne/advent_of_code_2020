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
            let mut pos_iter = iter
                .next()
                .unwrap()
                .split('-')
                .filter_map(|x| x.parse::<usize>().ok());
            let (first_pos, second_pos) = (pos_iter.next().unwrap(), pos_iter.next().unwrap());
            let char_to_find = iter.next().unwrap().chars().next().unwrap();
            let string = iter.next().unwrap();
            let output = (string.chars().nth(first_pos - 1) == Some(char_to_find)) ^ (string.chars().nth(second_pos - 1) == Some(char_to_find));
            // println!("for {}, it is {}", x, if output { "valid" } else { "invalid" });
            output
        })
        .count();
    println!("Number of valid passwords: {}", count);
}
