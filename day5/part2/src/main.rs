use std::io;
use std::io::prelude::*;

const NUM_ROWS: usize = 128;
const NUM_COLUMNS: usize = 8;

fn main() {
    let stdin = io::stdin();
    let vec: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();
    let mut ticket_ids: Vec<usize> =
        vec.iter()
            .map(|val| {
                let row = val.get(..NUM_COLUMNS - 1).unwrap().chars().fold(
                    (0, NUM_ROWS - 1),
                    |acc, c| match c {
                        'F' => (acc.0, (acc.0 + acc.1) / 2),
                        _ => ((acc.0 + acc.1) / 2 + 1, acc.1),
                    },
                );
                let column = val.get(NUM_COLUMNS - 1..).unwrap().chars().fold(
                    (0, NUM_COLUMNS - 1),
                    |acc, c| match c {
                        'L' => (acc.0, (acc.0 + acc.1) / 2),
                        _ => ((acc.0 + acc.1) / 2 + 1, acc.1),
                    },
                );
                assert_eq!(row.0, row.1);
                assert_eq!(column.0, column.1);
                row.0 * NUM_COLUMNS + column.0
            })
            .collect();
    ticket_ids.sort_unstable();
    let min = ticket_ids.iter().min().unwrap();
    let max = ticket_ids.iter().max().unwrap();
    let range_of_ids: Vec<usize> = (*min..*max).collect();
    let ticket_missing = ticket_ids
        .iter()
        .zip(range_of_ids.iter())
        .find(|(&actual_id, &expected_id)| actual_id != expected_id)
        .unwrap()
        .1;
    println!("Ticket missing is: {}", ticket_missing);
}
