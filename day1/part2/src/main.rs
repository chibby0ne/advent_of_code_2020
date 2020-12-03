use std::io;
use std::io::prelude::*;

fn find_pair(vec: &[i64], sum: i64) -> Option<(i64, i64)> {
    let mut iter = vec.iter();
    let mut iter_rev = vec.iter().rev();
    let mut iter_val = iter.next();
    let mut iter_rev_val = iter_rev.next();
    while iter.clone().lt(iter_rev.clone()) {
        match (iter_val, iter_rev_val) {
            (Some(&first), Some(&second)) => match (first + second).cmp(&sum) {
                std::cmp::Ordering::Greater => iter_rev_val = iter_rev.next(),
                std::cmp::Ordering::Less => iter_val = iter.next(),
                std::cmp::Ordering::Equal => return Some((first, second)),
            },
            _ => break,
        }
    }
    None
}

fn find_triplets(vec: &[i64]) -> Option<(i64, i64, i64)> {
    for (i, &first_factor) in vec.iter().enumerate() {
        let res = find_pair(&vec[i + 1..], 2020 - first_factor);
        if let Some(res) = res {
            return Some((first_factor, res.0, res.1));
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut vec: Vec<i64> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .into_iter()
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .into_iter()
        .collect();
    vec.sort_unstable();
    let triplets = find_triplets(&vec);
    if let Some(res) = triplets {
        println!(
            "Triplets are: {}, {}, {}, and product is: {}",
            res.0,
            res.1,
            res.2,
            res.0 * res.1 * res.2
        );
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_iter_eq() {
        let v: Vec<i64> = vec![0, 1, 2, 3];
        let mut rev_iter = v.iter().rev();
        let mut iter = v.iter();
        assert_eq!(iter.next().lt(&rev_iter.next()), true);
        assert_eq!(iter.next().lt(&rev_iter.next()), true);
        assert_eq!(iter.next().lt(&rev_iter.next()), false);
    }
}
