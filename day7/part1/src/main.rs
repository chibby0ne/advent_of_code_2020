use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;

const BAG: &str = "shinygold";

#[derive(Debug)]
struct Bags {
    bags: HashSet<Bag>,
}

impl Bags {
    fn new(bags: HashSet<Bag>) -> Bags {
        Bags { bags }
    }
}

#[derive(Debug, Eq)]
struct Bag {
    name: String,
    bags_and_numbers: HashSet<(String, i64)>,
}

impl Bag {
    fn new(bag_name: String) -> Bag {
        Bag {
            name: bag_name,
            bags_and_numbers: HashSet::new(),
        }
    }

    fn can_contain_bag(&self, bag: &str) -> bool {
        self.bags_and_numbers.iter().any(|(x, _)| x == bag)
    }

    fn add_contained_bag(&mut self, bag_and_number: (String, i64)) {
        self.bags_and_numbers.insert(bag_and_number);
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Bag) -> bool {
        self.name == other.name
    }
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn is_capable_of_holding(
    bags: &HashMap<String, Bag>,
    bag: &Bag,
    name_of_bag_to_hold: &str,
) -> bool {
    if bag.can_contain_bag(name_of_bag_to_hold) {
        return true;
    }
    let mut stack = vec![];
    stack.push(bag);
    while !stack.is_empty() {
        let b = stack.pop().unwrap();
        if b.can_contain_bag(name_of_bag_to_hold) {
            return true;
        } else {
            b.bags_and_numbers
                .iter()
                .for_each(|(x, _)| stack.push(bags.get(x).unwrap()));
        }
    }
    false
}

fn part1(bags: &HashMap<String, Bag>) {
    let mut cache: HashSet<&str> = HashSet::new();
    for (name, bag) in bags {
        if is_capable_of_holding(bags, bag, BAG) {
            cache.insert(name);
        }
    }
    println!("part1 count: {}", cache.iter().count());
}

fn main() {
    let stdin = io::stdin();
    let mut bags: HashMap<String, Bag> = HashMap::new();
    while let Some(Ok(line)) = stdin.lock().lines().next() {
        let mut iter = line.split("contain");
        let bag_name: String = iter
            .next()
            .unwrap()
            .trim()
            .trim_end_matches("bags")
            .trim()
            .split_whitespace()
            .collect();
        let iter_bags = iter.next().unwrap().split(',');
        let mut bag = Bag::new(bag_name.clone());
        for bag_contained in iter_bags {
            let mut bag_container_iter = bag_contained.trim().split_whitespace();
            if let Ok(number) = bag_container_iter.next().unwrap().parse::<i64>() {
                let bag_contained_name: String = bag_container_iter.take(2).collect();
                bag.add_contained_bag((bag_contained_name, number));
            }
        }
        bags.insert(bag_name, bag);
    }
    part1(&bags);
}
