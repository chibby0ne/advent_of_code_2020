use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;


// Lesson here after looking around how people do it:
// Better to have one a hacky solution than no elegant solution.
// Think about how you would find these relationships

// #[macro_use]
// extern crate lazy_static;
use regex::Regex;

fn main() {
    // lazy_static! {
    //     static ref re: Regex = Regex::new(r"[[:digit:]]+ (?P<color>[[:alpha:]]+ [[:alpha:]]+) bags?(?:,|\.)").unwrap();
    // }
    let re: Regex = Regex::new(r"[[:digit:]]+ (?P<color>[[:alpha:]]+ [[:alpha:]]+) bags?(?:,|\.)").unwrap();

    let stdin = io::stdin();
    let vec: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for line in vec {
        let bag_line: Vec<String> = line.split("contains").map(|x| x.to_string()).collect();
        let bag_key = bag_line.get(0).unwrap().strip_suffix("bags").unwrap();

        for cap in re.captures_iter(bag_line.get(1).unwrap()) {

        }
    

        let bag_value = String::from("asdf");
        match map.get_mut(bag_key) {
            Some(hashset) => {
                hashset.insert(bag_value);
            },
            None => {
                let mut m: HashSet<String> = HashSet::new();
                m.insert(bag_value);
                map.insert(bag_key.to_owned(), m);
            },

        }

    }
}

