// The shuttle company is running a contest: one gold coin for anyone that can find the earliest
// timestamp such that the first bus ID departs at that time and each subsequent listed bus ID
// departs at that subsequent minute. (The first line in your input is no longer relevant.)

// For example, suppose you have the same list of bus IDs as above:

// 7,13,x,x,59,x,31,19

// An x in the schedule means there are no constraints on what bus IDs must depart at that time.

// This means you are looking for the earliest timestamp (called t) such that:

//     Bus ID 7 departs at timestamp t.
//     Bus ID 13 departs one minute after timestamp t.
//     There are no requirements or restrictions on departures at two or three minutes after timestamp t.
//     Bus ID 59 departs four minutes after timestamp t.
//     There are no requirements or restrictions on departures at five minutes after timestamp t.
//     Bus ID 31 departs six minutes after timestamp t.
//     Bus ID 19 departs seven minutes after timestamp t.

// The only bus departures that matter are the listed bus IDs at their specific offsets from t.
// Those bus IDs can depart at other times, and other bus IDs can depart at those times. For
// example, in the list above, because bus ID 19 must depart seven minutes after the timestamp at
// which bus ID 7 departs, bus ID 7 will always also be departing with bus ID 19 at seven minutes
// after timestamp t.

// In this example, the earliest timestamp at which this occurs is 1068781:

// time     bus 7   bus 13  bus 59  bus 31  bus 19
// 1068773    .       .       .       .       .
// 1068774    D       .       .       .       .
// 1068775    .       .       .       .       .
// 1068776    .       .       .       .       .
// 1068777    .       .       .       .       .
// 1068778    .       .       .       .       .
// 1068779    .       .       .       .       .
// 1068780    .       .       .       .       .
// 1068781    D       .       .       .       .
// 1068782    .       D       .       .       .
// 1068783    .       .       .       .       .
// 1068784    .       .       .       .       .
// 1068785    .       .       D       .       .
// 1068786    .       .       .       .       .
// 1068787    .       .       .       D       .
// 1068788    D       .       .       .       D
// 1068789    .       .       .       .       .
// 1068790    .       .       .       .       .
// 1068791    .       .       .       .       .
// 1068792    .       .       .       .       .
// 1068793    .       .       .       .       .
// 1068794    .       .       .       .       .
// 1068795    D       D       .       .       .
// 1068796    .       .       .       .       .
// 1068797    .       .       .       .       .

// In the above example, bus ID 7 departs at timestamp 1068788 (seven minutes after t). This is
// fine; the only requirement on that minute is that bus ID 19 departs then, and it does.

// Here are some other examples:

//     The earliest timestamp that matches the list 17,x,13,19 is 3417.
//     67,7,59,61 first occurs at timestamp 754018.
//     67,x,7,59,61 first occurs at timestamp 779210.
//     67,7,x,59,61 first occurs at timestamp 1261476.
//     1789,37,47,1889 first occurs at timestamp 1202161486.

// However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger
// than 100000000000000!

// What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching
// their positions in the list?
//
//
//
// Algorithm:
// Calculate all the expected residues of the rest of the numbers X_1...X_N
// (which is X_i - i, where i = 1..N)
//
// Focus on finding a multiple of the first number X_0, increasingly from 1..Inf let's call the multiple T
//
// The residue of T % X_i where i = 1..N, should be, T % X_i = X_i - i
// (This left part of the equation can already be precomputed in the first step)
// Once that condition is satisfied for all numbers, that T is the earliest timestamp
//
//
// UPDATE: This is a brute force approach that simply takes too long to find for the real input.
// After some search, we find that this number theory problem (number theory is the branch of
// mathematics concerned with study of integers and integer valued functions), then this could be
// described as a diophantine equation, which are polynomial equations usually involving 2 or more
// variables such that the solutions of interest are integer ones, and deals with
// coprime (a.k.a setwise coprime) and more specifically: pairwise coprime numbers, then the
// Chinese Remainder Theorem can be used to solve it.
//
// Looking at the methods for solving the Chinese Remainder Theorem we can use the sieve approach.
//
// UPDATE2: The sieve approach is too slow still (O(2^N) exponential) :\ Better to use the
// existence construction approach:
// https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Using_the_existence_construction (which
// has a running of O(n^2))

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::cmp::Reverse;

// fn is_earliest_timestamp(timestamp: i128, bus_ids_and_residues: &[(i128, i128)]) -> bool {
//     for &(bus_id, residue) in bus_ids_and_residues {
//         if timestamp % bus_id != residue {
//             return false;
//         }
//     }
//     true
// }

fn find_earliest_timestamp(file: &File) -> Result<i128, Box<dyn Error>> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.lines().map(|x| x.to_owned()).collect();
    let mut bus_ids_and_expected_residues: Vec<(i128, i128)> = lines[1]
        .split(',')
        .enumerate()
        .map(|(i, x)| (i, x.parse::<i128>().ok()))
        .filter(|(_, x)| x.is_some())
        .map(|(i, x)| (i, x.unwrap()))
        .map(|(i, x)| {
            if i == 0 {
                (x, 0)
            } else {
                (x, (x - i as i128).abs())
            }
        })
        .collect();
    bus_ids_and_expected_residues.sort_unstable_by_key(|&(id, _)| Reverse(id));
    dbg!(&bus_ids_and_expected_residues);
    let top: i128 = bus_ids_and_expected_residues
        .iter()
        .fold(1, |mut acc, &(bus_id, _)| {
            acc *= bus_id;
            acc
        });
    let mut possible_timestamps: Vec<i128> = Vec::new();
    // Initialize with remainder of the largest modulus's congruence
    let mut timestamp: i128 = bus_ids_and_expected_residues[0].1;
    println!("initial timestamp: {}, top: {}", timestamp, top);

    // Fill in the values for the largest modulus
    while timestamp < top {
        possible_timestamps.push(timestamp);
        timestamp += bus_ids_and_expected_residues[0].0
    }
    dbg!(&possible_timestamps);

    let mut multiple = bus_ids_and_expected_residues[0].0;
    let mut last_index = 0;

    for &(id, remainder)  in bus_ids_and_expected_residues.iter().skip(1) {
        dbg!(id, remainder);
        for index in last_index..possible_timestamps.len() {
            dbg!(index);
            if possible_timestamps[index] % id == remainder {
                last_index = dbg!(index);
                timestamp = dbg!(possible_timestamps[index]);
                break;
            }
        }
        multiple *= id;
        dbg!(multiple);
        // Make the possible_timestamps smaller
        possible_timestamps = possible_timestamps.iter().skip(last_index).enumerate().map(|(i, &x)| {
            println!("{} == {} + {} * {}", x, timestamp, multiple, i);
            timestamp + multiple * i as i128
        })
        // .map(|(_, &x)| x)
        .collect();
        last_index = 0;
        dbg!(&possible_timestamps);

    }
    Ok(timestamp)


        // while timestamp < top {
        //     timestamp += bus_ids_and_expected_residues[0].0;
        //     if is_earliest_timestamp(timestamp, &bus_ids_and_expected_residues[1..]) {
        //         return Ok(timestamp);
        //     }
        // }
        // Err(Box::from("Could not find earliest timestamp"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // #[ignore]
    #[test]
    fn test_example_input() -> Result<(), Box<dyn Error>> {
        let path = env::current_dir()?;
        let file = File::open(path.join("../example_input"))?;
        assert_eq!(1068781, find_earliest_timestamp(&file)?);
        Ok(())
    }

    // #[ignore]
    #[test]
    fn test_example_input2() -> Result<(), Box<dyn Error>> {
        let path = env::current_dir()?;
        let file = File::open(path.join("../example_input2"))?;
        assert_eq!(3417, find_earliest_timestamp(&file)?);
        Ok(())
    }

    // #[ignore]
    #[test]
    fn test_example_input3() -> Result<(), Box<dyn Error>> {
        let path = env::current_dir()?;
        let file = File::open(path.join("../example_input3"))?;
        assert_eq!(754018, find_earliest_timestamp(&file)?);
        Ok(())
    }

    // #[ignore]
    #[test]
    fn test_example_input4() -> Result<(), Box<dyn Error>> {
        let path = env::current_dir()?;
        let file = File::open(path.join("../example_input4"))?;
        assert_eq!(779210, find_earliest_timestamp(&file)?);
        Ok(())
    }

    // #[ignore]
    // #[test]
    // fn test_input() -> Result<(), Box<dyn Error>> {
    //     let path = env::current_dir()?;
    //     let file = File::open(path.join("../input"))?;
    //     assert_eq!(779210, find_earliest_timestamp(&file)?);
    //     Ok(())
    // }
}
