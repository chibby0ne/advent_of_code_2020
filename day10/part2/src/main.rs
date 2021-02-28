// --- Part Two ---

// To completely determine whether you have enough adapters, you'll need to figure out how many different ways they can be arranged. Every arrangement needs to connect the charging outlet to your device. The previous rules about when adapters can successfully connect still apply.

// The first example above (the one that starts with 16, 10, 15) supports the following arrangements:

// (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
// (0), 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, (22)
// (0), 1, 4, 5, 7, 10, 11, 12, 15, 16, 19, (22)
// (0), 1, 4, 5, 7, 10, 12, 15, 16, 19, (22)
// (0), 1, 4, 6, 7, 10, 11, 12, 15, 16, 19, (22)
// (0), 1, 4, 6, 7, 10, 12, 15, 16, 19, (22)
// (0), 1, 4, 7, 10, 11, 12, 15, 16, 19, (22)
// (0), 1, 4, 7, 10, 12, 15, 16, 19, (22)

// (The charging outlet and your device's built-in adapter are shown in parentheses.) Given the adapters from the first example, the total number of arrangements that connect the charging outlet to your device is 8.

// The second example above (the one that starts with 28, 33, 18) has many arrangements. Here are a few:

// (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
// 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)

// (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
// 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 49, (52)

// (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
// 32, 33, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)

// (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
// 32, 33, 34, 35, 38, 39, 42, 45, 46, 49, (52)

// (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
// 32, 33, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)

// (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
// 46, 48, 49, (52)

// (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
// 46, 49, (52)

// (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
// 47, 48, 49, (52)

// (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
// 47, 49, (52)

// (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
// 48, 49, (52)

// In total, this set of adapters can connect the charging outlet to your device in 19208 distinct arrangements.

// You glance back down at your bag and try to remember why you brought so many adapters; there must be more than a trillion valid ways to arrange them! Surely, there must be an efficient way to count the arrangements.

// What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?

use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut input: Vec<i64> = stdin
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    let num_chargers = input.len();
    input.sort_unstable();
    input.dedup();

    let mut prev: i64 = 0; // seat outlet jolt rating
    let mut i = 0;
    let mut new_vec: Vec<i64> = Vec::new();

    // input.push(input.iter().max().unwrap() + 3); // device's adapter

    while i < input.len() {
        if let Some(&val) = input.get(i) {
            match val - prev {
                3 => {
                    println!("Adding val: {}, which is i and has 3 diff", val);
                    new_vec.push(val)
                },
                2 => {
                    if let Some(&v) = input.get(i + 1) {
                        match v - prev {
                            3 => {
                                println!("Adding v: {}, which is i + 1 and has 3 diff", v);
                                new_vec.push(v);
                                i += 1;
                            },
                            _ => {
                                println!("Adding val: {}, which is i and has 2 diff", v);
                                new_vec.push(val);
                            }
                        }
                    }
                },
                _ => {
                    if let Some(&v) = input.get(i + 2) {
                        match v - prev {
                            2..=3 => {
                                println!("Adding v: {}, which is in i + 2, and has 2..=3 diff", v);
                                new_vec.push(v);
                                i += 2;
                            }
                            _ => {
                                if let Some(&vv) = input.get(i + 1) {
                                    match vv - prev {
                                        3 => {
                                            println!("Adding val: {} which is in i + 1, and has 3 diff", vv);
                                            new_vec.push(vv);
                                            i += 1;
                                        },
                                        _ => {
                                            println!("Adding val: {}, which is i, and has a 1 diff", val);
                                            new_vec.push(val);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

            }
            i += 1;
            prev = val;
        }
    }
    dbg!(&input);
    dbg!(&new_vec);

    let diff = num_chargers - new_vec.len();

    println!("{}", diff * (diff - 1) + 1 + 1);
}
