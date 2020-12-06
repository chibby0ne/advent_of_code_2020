use std::io;
use std::io::prelude::*;

const FIELDS: [&str; 7] = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

const VALID_YEAR_DIGITS: usize = 4;

const LOWER_BOUND_BIRTH_YEAR: usize = 1920;
const UPPER_BOUND_BIRTH_YEAR: usize = 2002;

const LOWER_BOUND_ISSUE_YEAR: usize = 2010;
const UPPER_BOUND_ISSUE_YEAR: usize = 2020;

const LOWER_BOUND_EXPIRATION_YEAR: usize = 2020;
const UPPER_BOUND_EXPIRATION_YEAR: usize = 2030;

const LOWER_BOUND_HEIGHT_CM: usize = 150;
const UPPER_BOUND_HEIGHT_CM: usize = 193;

const LOWER_BOUND_HEIGHT_IN: usize = 59;
const UPPER_BOUND_HEIGHT_IN: usize = 76;

const VALID_HCL_DIGITS: usize = 6;

const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

const VALID_PID_DIGITS: usize = 9;

const SKIP: &str = "skip";

fn return_last_state(i: usize, length: usize, state: &str) -> Option<String> {
    if i == length - 1 {
        Some(state.to_string())
    } else {
        Some(SKIP.to_string())
    }
}

fn is_valid_year(year_str: &str, lower_bound: usize, upper_bound: usize) -> bool {
    let year_num: Option<usize> = year_str.parse().ok();
    match year_num {
        Some(year) => {
            year >= lower_bound
                && year <= upper_bound
                && year_str.chars().count() == VALID_YEAR_DIGITS
        }
        _ => false,
    }
}

fn is_valid_byr(byr: &str) -> bool {
    is_valid_year(byr, LOWER_BOUND_BIRTH_YEAR, UPPER_BOUND_BIRTH_YEAR)
}

fn is_valid_iyr(iyr: &str) -> bool {
    is_valid_year(iyr, LOWER_BOUND_ISSUE_YEAR, UPPER_BOUND_ISSUE_YEAR)
}

fn is_valid_eyr(eyr: &str) -> bool {
    is_valid_year(
        eyr,
        LOWER_BOUND_EXPIRATION_YEAR,
        UPPER_BOUND_EXPIRATION_YEAR,
    )
}

fn is_valid_height_and_range(
    hgt: &str,
    unit: &str,
    lower_bound: usize,
    upper_bound: usize,
) -> bool {
    match hgt.strip_suffix(unit) {
        Some(height_str) => match height_str.parse::<usize>().ok() {
            Some(height) => height >= lower_bound && height <= upper_bound,
            _ => false,
        },
        _ => false,
    }
}

fn is_valid_hgt(hgt: &str) -> bool {
    match hgt.get(hgt.len() - 2..) {
        Some("cm") => {
            is_valid_height_and_range(hgt, "cm", LOWER_BOUND_HEIGHT_CM, UPPER_BOUND_HEIGHT_CM)
        }
        Some("in") => {
            is_valid_height_and_range(hgt, "in", LOWER_BOUND_HEIGHT_IN, UPPER_BOUND_HEIGHT_IN)
        }
        _ => false,
    }
}

fn is_valid_hcl(hcl: &str) -> bool {
    match hcl.strip_prefix('#') {
        Some(hcl_stripped) => {
            hcl_stripped.len() == VALID_HCL_DIGITS && i64::from_str_radix(hcl_stripped, 16).is_ok()
        }
        _ => false,
    }
}

fn is_valid_ecl(ecl: &str) -> bool {
    VALID_EYE_COLORS.contains(&ecl)
}

fn is_valid_pid(pid: &str) -> bool {
    pid.len() == VALID_PID_DIGITS && pid.parse::<usize>().is_ok()
}

fn main() {
    let stdin = io::stdin();
    let vec: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    let count = vec
        .iter()
        .enumerate()
        .scan(String::new(), |state, (i, val)| {
            let num_fields = val.split_whitespace().count();
            if num_fields == 0 {
                let res = state.clone();
                *state = "".to_string();
                Some(res)
            } else if state.is_empty() {
                *state = val.to_string();
                return_last_state(i, vec.len(), &state)
            } else {
                let line = format!("{} {}", *state, val);
                *state = line;
                return_last_state(i, vec.len(), &state)
            }
        })
        .filter(|x| x != SKIP)
        .filter(|x| FIELDS.iter().all(|&key| x.contains(key)))
        .filter(|x| {
            x.split_whitespace().all(|field| {
                let key = field.split(':').next();
                let value = field.split(':').nth(1).unwrap();
                match key {
                    Some("byr") => is_valid_byr(&value),
                    Some("iyr") => is_valid_iyr(&value),
                    Some("eyr") => is_valid_eyr(&value),
                    Some("hgt") => is_valid_hgt(&value),
                    Some("hcl") => is_valid_hcl(&value),
                    Some("ecl") => is_valid_ecl(&value),
                    Some("pid") => is_valid_pid(&value),
                    Some("cid") => true,
                    _ => false,
                }
            })
        })
        .count();

    println!("Number of valid passports is: {}", count);
}
