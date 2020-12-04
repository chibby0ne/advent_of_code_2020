use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let vec: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();
    let mut pos = 0;
    let count = vec
        .iter()
        .skip(1)
        .filter(|&x| {
            // tuple of two lines looks like this (for example):
            // ...#....#
            // ..#..#...
            let mut chars = x.chars().cycle();
            pos += 3;
            let result = chars.nth(pos) == Some('#');
            println!("for line: {}, {}", x, if result { "X" } else { "O" });
            result
        })
        .count();
    println!("Number of trees encountered: {}", count);
}
