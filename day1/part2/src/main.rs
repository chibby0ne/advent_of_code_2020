use std::io;
use std::io::prelude::*;

fn find_pair(vec: &[i64], sum: i64) -> Option<(i64, i64)> {
    let mut iter = vec.iter();
    match (iter.nth(0), iter.last()) {
         (Some(&first), Some(&second)) => {
             if first + second == sum {
                 Some((first, second))
             } else {
                 None
             }
         },
         _ => None,
    }
}


fn find_triplets(vec: &Vec<i64>) -> Option<(i64, i64, i64)> {
    for (i, &first_factor) in vec.iter().enumerate() {
        let res = find_pair(&vec[i + 1..], 2020 - first_factor);
        match res {
            Some(res) => return Some((first_factor, res.0, res.1)),
            _ => (),
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let vec: Vec<i64> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .into_iter()
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .into_iter()
        .collect();
    let triplets = find_triplets(&vec);
    match triplets {
        Some(res) => println!("Triplets are: {}, {}, {}, and product is: {}", res.0, res.1, res.2, res.0 * res.1 * res.2),
        _ => (),
    }
}
