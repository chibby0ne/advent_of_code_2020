// --- Day 16: Ticket Translation ---
//
// As you're walking to yet another connecting flight, you realize that one of the legs of your re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in a language you don't understand. You should probably figure out what it says before you get to the train station after the next flight.
//
// Unfortunately, you can't actually read the words on the ticket. You can, however, read the numbers, and so you figure out the fields these tickets must have and the valid ranges for values in those fields.
//
// You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other nearby tickets for the same train service (via the airport security cameras) together into a single document you can reference (your puzzle input).
//
// The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the valid ranges of values for each field. For example, a rule like class: 1-3 or 5-7 means that one of the fields in every ticket is named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such that 3 and 5 are both valid in this field, but 4 is not).
//
// Each ticket is represented by a single line of comma-separated values. The values are the numbers on the ticket in the order they appear; every ticket has the same format. For example, consider this ticket:
//
// .--------------------------------------------------------.
// | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
// |                                                        |
// | ??: 301  ??: 302             ???????: 303      ??????? |
// | ??: 401  ??: 402           ???? ????: 403    ????????? |
// '--------------------------------------------------------'
//
// Here, ? represents text in a language you don't understand. This ticket might be represented as 101,102,103,104,301,302,303,401,402,403; of course, the actual train tickets you're looking at are much more complicated. In any case, you've extracted just the numbers in such a way that the first number is always the same specific field, the second number is always a different specific field, and so on - you just don't know what each position actually means!
//
// Start by determining which tickets are completely invalid; these are tickets that contain values which aren't valid for any field. Ignore your ticket for now.
//
// For example, suppose you have the following notes:
//
// class: 1-3 or 5-7
// row: 6-11 or 33-44
// seat: 13-40 or 45-50
//
// your ticket:
// 7,1,14
//
// nearby tickets:
// 7,3,47
// 40,4,50
// 55,2,20
// 38,6,12
//
// It doesn't matter which position corresponds to which field; you can identify invalid nearby tickets by considering only whether tickets contain values that are not valid for any field. In this example, the values on the first nearby ticket are all valid for at least one field. This is not true of the other three nearby tickets: the values 4, 55, and 12 are are not valid for any field. Adding together all of the invalid values produces your ticket scanning error rate: 4 + 55 + 12 = 71.
//
// Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate?
//
// To begin, get your puzzle input.

//
// --- Part Two ---
//
// Now that you've identified which tickets contain invalid values, discard those tickets entirely. Use the remaining valid tickets to determine which field is which.
//
// Using the valid ranges for each field, determine what order the fields appear on the tickets. The order is consistent between all tickets: if seat is the third field, it is the third field on every ticket, including your ticket.
//
// For example, suppose you have the following notes:
//
// class: 0-1 or 4-19
// row: 0-5 or 8-19
// seat: 0-13 or 16-19
//
// your ticket:
// 11,12,13
//
// nearby tickets:
// 3,9,18
// 15,1,5
// 5,14,9
//
// Based on the nearby tickets in the above example, the first position must be row, the second position must be class, and the third position must be seat; you can conclude that in your ticket, class is 12, row is 11, and seat is 13.
// Once you work out which field is which, look for the six fields on your ticket that start with the word departure. What do you get if you multiply those six values together?
//

#![allow(dead_code)]
#![allow(unused)]

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, stdin};
use std::ops::RangeInclusive;

const RANGE_REGEX: &str = r"(\d+)-(\d+)";
const RULE_REGEX: &str = r"(.+):";

#[derive(Debug, PartialEq)]
struct Rule {
    ranges: Vec<RangeInclusive<i64>>,
}

impl Rule {
    fn new(rule: &str) -> Self {
        let re = Regex::new(RANGE_REGEX).unwrap();
        let ranges = re
            .captures_iter(rule)
            .map(|cap| {
                let start = cap[1].parse().unwrap();
                let end = cap[2].parse().unwrap();
                RangeInclusive::new(start, end)
            })
            .collect();
        Self { ranges }
    }

    /// Mutates the Ticket such that the values that pass this rule are removed from the
    /// unverified_values of the ticket
    /// returns true if the ticket is already valid (early termination) or if the ticket became
    /// valid after the application of this rule
    /// In addition it mutates the Ticket by setting the is_valid flag to true in the latter case
    fn validate(&self, t: &mut Ticket) -> bool {
        if t.is_valid {
            return true;
        }
        t.unverified_values.retain(|val| {
            for range in self.ranges.iter() {
                if range.start() <= val && val <= range.end() {
                    return false;
                }
            }
            true
        });
        if t.unverified_values.is_empty() {
            t.is_valid = true;
        }
        t.is_valid
    }

    fn fits(&self, v: &i64) -> bool {
        for range in self.ranges.iter() {
            if range.start() <= v && v <= range.end() {
                return true;
            }
        }
        false
    }

    fn fits_vector(&self, v: &Vec<i64>) -> bool {
        v.iter().all(|x| self.fits(x))
    }
}

#[derive(Debug, PartialEq)]
struct RuleSet {
    rules: HashMap<String, Rule>,
}

impl RuleSet {
    fn new(v: Vec<String>) -> Self {
        let re = Regex::new(RULE_REGEX).unwrap();
        let mut rules = HashMap::new();
        for elem in v {
            let name = re.captures(&elem).map(|cap| cap[1].to_string()).unwrap();
            let _r = Rule::new(&elem);
            rules.insert(name, _r);
        }
        Self { rules }
    }

    /// Applies the `validate` method of as many rules as it needs to verify that the ticket is valid
    fn is_valid_ticket(&self, t: &mut Ticket) -> bool {
        self.rules.values().any(|x| x.validate(t))
    }
}

#[derive(Debug, PartialEq, Default)]
struct Ticket {
    unverified_values: Vec<i64>,
    copy_values: Vec<i64>,
    is_valid: bool,
}

impl Ticket {
    fn new(v: &str) -> Self {
        Self {
            unverified_values: v.split(',').filter_map(|x| x.parse().ok()).collect(),
            copy_values: v.split(',').filter_map(|x| x.parse().ok()).collect(),
            ..Default::default()
        }
    }

    fn error_rate(&self) -> i64 {
        self.unverified_values.iter().sum()
    }
}

fn split_input_into_rules_and_tickets(input: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut sections = input.split(|x| x.is_empty());
    let rules = sections.next().unwrap();
    let tickets = sections
        .next_back()
        .unwrap()
        .iter()
        .skip(1)
        .cloned()
        .collect();
    (rules.to_vec(), tickets)
}

/// Get all valid tickets from list of tickets and ruleset
fn get_all_valid_tickets<'a>(tickets: &'a mut Vec<Ticket>, ruleset: &RuleSet) -> Vec<&'a Ticket> {
    let mut valid_tickets: Vec<&Ticket> = Vec::new();
    for ticket in tickets.iter_mut() {
        if ruleset.is_valid_ticket(ticket) {
            valid_tickets.push(ticket);
        }
    }
    valid_tickets
}

/// Create a Vec<Vec<i64>> which is a matrix of values where the rows represent the same field,
/// and the columns represent a specific ticket example
fn create_matrix_of_fields_from_tickets(valid_tickets: &Vec<&Ticket>) -> Vec<Vec<i64>> {
    let mut fields_for_all_tickets: Vec<Vec<i64>> = Vec::new();

    let mut field_index = 0;
    let number_fields = valid_tickets.first().unwrap().copy_values.len();

    while field_index < number_fields {
        let mut new_vec: Vec<i64> = Vec::new();

        let mut ticket_index = 0;
        let number_tickets = valid_tickets.len();

        while ticket_index < number_tickets {
            let ticket = valid_tickets.get(ticket_index).unwrap();
            let field_value = ticket.copy_values.get(field_index).unwrap();
            new_vec.push(*field_value);
            ticket_index += 1
        }
        fields_for_all_tickets.push(new_vec);
        field_index += 1;
    }
    fields_for_all_tickets
}

fn main() -> Result<(), io::Error> {
    let lines = stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let (rules_lines, tickets_lines) = split_input_into_rules_and_tickets(lines.clone());
    let ruleset = RuleSet::new(rules_lines);
    let mut tickets: Vec<_> = tickets_lines.into_iter().map(|x| Ticket::new(&x)).collect();
    let valid_tickets = get_all_valid_tickets(&mut tickets, &ruleset);

    let fields_for_all_tickets = create_matrix_of_fields_from_tickets(&valid_tickets);
    let mut field_names_to_field_indexes: HashMap<&str, i64> = HashMap::new();

    let number_of_tickets = fields_for_all_tickets.first().unwrap().len();
    let number_of_fields = fields_for_all_tickets.len();
    eprintln!(
        "num_tickets: {}, fields: {}",
        number_of_tickets, number_of_fields
    );

    // Find all the field names using the valid tickets and their ranges
    // Known and unknown field_names
    let mut unknown: HashSet<&String> = ruleset.rules.keys().collect();
    // Known and unknown field_indexes
    let mut indexes_unknown: HashSet<usize> = (0..number_of_fields).collect();
    while !unknown.is_empty() {
        // Find the next field that can only fit a single rule
        let rule_name = *unknown.iter().next().unwrap();
        let rule = ruleset.rules.get(rule_name).unwrap();
        let mut iter = indexes_unknown.iter();
        loop {
            let i = *iter.next().unwrap();
            let field = fields_for_all_tickets.get(i).unwrap();
            eprintln!("checking {rule_name} with field {i}");
            if rule.fits_vector(field)
                && fields_for_all_tickets.iter().all(|v| {
                    if v == field {
                        true
                    } else {
                        !rule.fits_vector(v)
                    }
                })
            {
                eprintln!("field {i} only fits with rule {rule_name}");
                unknown.remove(rule_name);
                indexes_unknown.remove(&i);
                field_names_to_field_indexes.insert(rule_name, i as i64);
                break;
            }
        }
    }
    dbg!(&field_names_to_field_indexes);

    // Get only the departure fields
    let mut field_indexes_of_departure_fields: Vec<i64> = Vec::new();
    for (key, field_index) in field_names_to_field_indexes {
        if key.starts_with("departure") {
            field_indexes_of_departure_fields.push(field_index);
        }
    }
    dbg!(&field_indexes_of_departure_fields);

    // Get my ticket
    let my_ticket = lines
        .windows(2)
        .find(|x| x[0] == "your ticket:")
        .map(|x| &x[1])
        .unwrap();
    dbg!(&my_ticket);
    let my_ticket_values: Vec<i64> = my_ticket
        .split(',')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    dbg!(&my_ticket_values);

    // Multiply all the departure fields of my ticket and print it
    let departure_values: Vec<&i64> = my_ticket_values
        .iter()
        .enumerate()
        .filter(|(i, x)| field_indexes_of_departure_fields.contains(&(*i as i64)))
        .map(|(_, x)| x)
        .collect();

    dbg!(&departure_values);

    let total: i64 = departure_values.iter().copied().product();

    // Print result
    println!("{}", total);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_input_into_rules_and_tickets() {
        let input_str = vec![
            "wagon: 31-165 or 176-962",
            "zone: 48-870 or 896-970",
            "",
            "your ticket:",
            "127,89,100",
            "",
            "nearby tickets:",
            "127,89,149,113,181,131,53,199,103,107,97,179,109,193,151,83,197,101,211,191",
            "835,933,819,240,276,334,830,786,120,791,301,770,249,767,177,84,838,85,596,352",
        ];
        let input = input_str.into_iter().map(|x| x.to_owned()).collect();

        let expected_rules = vec![
            "wagon: 31-165 or 176-962".to_string(),
            "zone: 48-870 or 896-970".to_string(),
        ];
        let expected_tickets = vec![
            "127,89,149,113,181,131,53,199,103,107,97,179,109,193,151,83,197,101,211,191"
                .to_string(),
            "835,933,819,240,276,334,830,786,120,791,301,770,249,767,177,84,838,85,596,352"
                .to_string(),
        ];

        let (actual_rules, actual_tickets) = split_input_into_rules_and_tickets(input);
        assert_eq!(expected_rules, actual_rules);
        assert_eq!(expected_tickets, actual_tickets);
    }

    #[test]
    fn test_new_rule() {
        let input = "wagon: 31-165 or 176-962";
        let expected_rule = Rule {
            ranges: vec![31..=165, 176..=962],
        };
        let actual_rule = Rule::new(&input);
        assert_eq!(expected_rule, actual_rule);
    }

    #[test]
    fn test_new_ruleset() {
        let input = vec!["wagon: 31-165 or 176-962".to_string()];
        let mut hashmap = HashMap::new();
        hashmap.insert("wagon".to_string(), Rule::new(&input[0]));
        let expected_ruleset = RuleSet { rules: hashmap };
        let actual_ruleset = RuleSet::new(input);
        assert_eq!(expected_ruleset, actual_ruleset);
    }

    #[test]
    fn test_regex() {
        let input = "wagon: 31-165 or 176-962";
        let re = Regex::new(r"(.+):").unwrap();
        let capture_opt = re.captures(input);
        assert!(capture_opt.is_some());
        let p = capture_opt.unwrap().get(1).map_or("", |m| m.as_str());
        assert_eq!("wagon", p);
    }

    #[test]
    fn test_pass_rule_false() {
        let rule = Rule::new("class: 1-3 or 5-7");
        let mut ticket = Ticket::new("7,3,47");
        let expected_ticket = Ticket::new("47");
        let expected_output = false;
        let actual_output = rule.validate(&mut ticket);
        assert_eq!(expected_ticket.unverified_values, ticket.unverified_values);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_pass_rule_true() {
        let rule = Rule::new("class: 1-3 or 5-70");
        let mut ticket = Ticket::new("7,3,47");
        let mut expected_ticket = Ticket::new("");
        let expected_output = true;
        let actual_output = rule.validate(&mut ticket);
        assert_eq!(expected_ticket.unverified_values, ticket.unverified_values);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_ruleset_is_valid_ticket_true() {
        let rules = vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
        ];
        let ruleset = RuleSet::new(rules);
        let mut ticket = Ticket::new("7,3,47");
        let expected_output = true;
        let actual_output = ruleset.is_valid_ticket(&mut ticket);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_ruleset_is_valid_ticket_false() {
        let rules = vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
        ];
        let ruleset = RuleSet::new(rules);
        let mut ticket = Ticket::new("40,4,50");
        let expected_output = false;
        let actual_output = ruleset.is_valid_ticket(&mut ticket);
        assert_eq!(expected_output, actual_output);
        let mut t = Ticket::new("40,4,50");
        t.unverified_values = vec![4];
        assert_eq!(t, ticket);
    }

    #[test]
    fn test_create_matrix_of_fields_from_tickets() {
        let mut tickets = vec![
            Ticket::new("1,6,13"),
            Ticket::new("3,11,40"),
            Ticket::new("5,33,45"),
            Ticket::new("6,34,46"),
        ];
        let rules = vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
        ];
        let ruleset = RuleSet::new(rules);
        let valid_tickets = get_all_valid_tickets(&mut tickets, &ruleset);
        let fields_for_all_tickets = create_matrix_of_fields_from_tickets(&valid_tickets);

        let expected: Vec<Vec<i64>> =
            vec![vec![1, 3, 5, 6], vec![6, 11, 33, 34], vec![13, 40, 45, 46]];

        assert_eq!(expected, fields_for_all_tickets);
    }
}
