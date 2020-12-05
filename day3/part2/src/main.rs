use std::io;
use std::io::prelude::*;

fn number_of_trees_encountered(vec: &[String], right: usize, down: usize) -> usize {
    let mut pos = 0;
    vec.iter()
        .skip(down)
        .step_by(down)
        .filter(|&x| {
            // tuple of two lines looks like this (for example):
            // ...#....#
            // ..#..#...
            let mut chars = x.chars().cycle();
            pos += right;
            let found = chars.nth(pos) == Some('#');
            println!("for {}: {}", x, if found { "X" } else { "O" });
            found
        })
        .count()
}

fn main() {
    let stdin = io::stdin();
    let vec: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product = 1;
    for (i, &slope) in slopes.iter().enumerate() {
        let trees = number_of_trees_encountered(&vec, slope.0, slope.1);
        println!("For slope: {}, trees encountered: {}", i, trees);
        product *= trees;
    }
    println!("Product: {}", product);
}
