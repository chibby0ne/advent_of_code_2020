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

    input.push(0); // seat outlet
    input.push(input.iter().max().unwrap() + 3); // built in adapter

    let num_chargers = input.len();
    input.sort_unstable();
    let len_before_dedup = input.len();
    input.dedup();
    let len_after_dedup = input.len();
    assert_eq!(len_before_dedup, len_after_dedup);

    let mut new_vec: Vec<i64> = vec![0];

    // input.push(input.iter().max().unwrap() + 3); // device's adapter
    //
    // let mut iter = input.iter().skip(1);
    // let mut past_iter = input.iter().take(input.len() - 1);

    // while let Some(&past) = dbg!(past_iter.next()) {
    //     if let Some(&current) = dbg!(iter.next()) {
    //         match dbg!(current - past) {
    //             3 => {
    //                 println!("was 3 pushing: {}", past);
    //                 new_vec.push(past);
    //             },
    //             2 => {
    //                 println!("was 2 checking next_current");
    //                 if let Some(&next_current) = iter.nth(1) {
    //                     match next_current - past {
    //                         3 => {
    //                             println!("was 3 the next current pushing {}", next_current);
    //                             new_vec.push(next_current);
    //                             past_iter.next(); 
    //                             iter.next();
    //                             println!("updated both iterators once");
    //                         },
    //                         _ => {
    //                             println!("was 2 and next_current wasn't good, pushing: {}", past);
    //                             new_vec.push(past);
    //                         }
    //                     }
    //                 }
    //             },
    //             _ => {
    //                 println!("was 1 checking next_next_current");
    //                 if let Some(&next_next_current) = iter.nth(2) {
    //                     match next_next_current - past {
    //                         3 => {
    //                             println!("was 3 the next_next_current pushing: {}", next_next_current); 
    //                             new_vec.push(next_next_current);
    //                             past_iter.next(); 
    //                             past_iter.next(); 
    //                             iter.next();
    //                             iter.next();
    //                             println!("updated both iterators twice");
    //                         },
    //                         _ => {
    //                             println!("was 1 and the next_next_current wasn't good enough, checking next_current"); 
    //                             if let Some(&next_current) = iter.nth(1) {
    //                                 match next_current - past {
    //                                     3 => {
    //                                         println!("was 3 the next_current pushing: {}", next_current);
    //                                         new_vec.push(next_current);
    //                                         past_iter.next(); 
    //                                         iter.next();
    //                                         println!("updated both iterators once");

    //                                     }
    //                                     _ => {
    //                                         println!("was 1 pushing: {}", past);
    //                                         new_vec.push(past);
    //                                     }

    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }


    let mut i = 1;
    let mut last_added = 0;

    while i < input.len() {
        if let Some(&curr) = input.get(i) {
            match dbg!(curr - last_added) {
                3 => {
                    println!("Adding: {} which is i has 3 diff", curr);
                    new_vec.push(curr);
                    last_added = curr;
                },
                2 => {
                    if let Some(&next_curr) = input.get(i + 1) {
                        match next_curr - last_added {
                            3 => {
                                println!("Adding v: {} which is i + 1 has 3 diff", next_curr);
                                new_vec.push(next_curr);
                                i += 1;
                                last_added = next_curr;
                            },
                            _ => {
                                println!("Adding val: {}, which is i has 2 diff", curr);
                                new_vec.push(curr);
                                last_added = curr;
                            }
                        }
                    }
                },
                _ => {
                    if let Some(&next_next_curr) = input.get(i + 2) {
                        match next_next_curr - last_added {
                            2..=3 => {
                                println!("Adding v: {}, which is in i + 2, and has 2..=3 diff", next_next_curr);
                                new_vec.push(next_next_curr);
                                i += 2;
                                last_added = next_next_curr;
                            }
                            _ => {
                                if let Some(&next_curr) = input.get(i + 1) {
                                    match next_curr - last_added {
                                        2..=3 => {
                                            println!("Adding val: {} which is in i + 1, and has 2..=3 diff", next_curr);
                                            new_vec.push(next_curr);
                                            i += 1;
                                            last_added = next_curr;
                                        },
                                        _ => {
                                            println!("Adding val: {}, which is i, and has a 1 diff", curr);
                                            new_vec.push(curr);
                                            last_added = curr;
                                        }
                                    }
                                }
                            }
                        }
                    } else if let Some(&next_curr) = input.get(i + 1) {
                        match next_curr - last_added {
                            2..=3 => {
                                println!("Adding val: {} which is in i + 1, and has 2..=3 diff", next_curr);
                                new_vec.push(next_curr);
                                i += 1;
                                last_added = next_curr;
                            },
                            _ => {
                                println!("Adding val: {}, which is i, and has a 1 diff", curr);
                                new_vec.push(curr);
                                last_added = curr;
                            }
                        }
                    }
                }

            }
            i += 1;
        }
    }
    dbg!(&input);
    dbg!(&new_vec);

    let diff = dbg!(num_chargers - new_vec.len());

    println!("{}", diff * (diff - 1) + 1 + 1);
}
