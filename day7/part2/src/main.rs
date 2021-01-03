use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;

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

fn calculate_bags_inside(bags: &HashMap<String, Bag>, bag_name: &String, sum_so_far: i64) -> i64 {
    let bag = bags.get(bag_name).unwrap();
    let mut sum = sum_so_far;
    for (inside_bag_name, inside_bag_num) in &bag.bags_and_numbers {
        sum += calculate_bags_inside(bags, &inside_bag_name, sum_so_far * inside_bag_num)
    }
    sum
}

fn part2(bags: &HashMap<String, Bag>) {
    let mut sum_outside = 0;
    let shinygold = bags.get("shinygold").unwrap();
    for (bag_name, number) in &shinygold.bags_and_numbers {
        sum_outside += calculate_bags_inside(bags, bag_name, *number);
    }
    println!("part2: count: {}", sum_outside);
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
    part2(&bags);
}
