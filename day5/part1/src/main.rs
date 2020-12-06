use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let vec: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();
    let max = vec
        .iter()
        .map(|val| {
            let row = val.get(..7).unwrap().chars().fold((0, 127), |acc, c| {
                dbg!(c);
                match c {
                    'F' => dbg!((acc.0, (acc.0 + acc.1) / 2)),
                    _ => dbg!(((acc.0 + acc.1) / 2 + 1, acc.1)),
                }
            });
            let column = val
                .get(7..)
                .unwrap()
                .chars()
                .fold((0, 7), |acc, c| match c {
                    'L' => dbg!((acc.0, (acc.0 + acc.1) / 2)),
                    _ => dbg!(((acc.0 + acc.1) / 2 + 1, acc.1)),
                });
            assert_eq!(row.0, row.1);
            assert_eq!(column.0, column.1);
            row.0 * 8 + column.0
        })
        .max()
        .unwrap();
    println!("The highest seat ID is: {}", max);
}
