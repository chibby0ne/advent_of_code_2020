// --- Part Two ---
//
// For some reason, the sea port's computer system still can't communicate with your ferry's docking program. It must be using version 2 of the decoder chip!
//
// A version 2 decoder chip doesn't modify the values being written at all. Instead, it acts as a memory address decoder. Immediately before a value is written to memory, each bit in the bitmask modifies the corresponding bit of the destination memory address in the following way:
//
//     If the bitmask bit is 0, the corresponding memory address bit is unchanged.
//     If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
//     If the bitmask bit is X, the corresponding memory address bit is floating.
//
// A floating bit is not connected to anything and instead fluctuates unpredictably. In practice, this means the floating bits will take on all possible values, potentially causing many memory addresses to be written all at once!
//
// For example, consider the following program:
//
// mask = 000000000000000000000000000000X1001X
// mem[42] = 100
// mask = 00000000000000000000000000000000X0XX
// mem[26] = 1
//
// When this program goes to write to memory address 42, it first applies the bitmask:
//
// address: 000000000000000000000000000000101010  (decimal 42)
// mask:    000000000000000000000000000000X1001X
// result:  000000000000000000000000000000X1101X
//
// After applying the mask, four bits are overwritten, three of which are different, and two of which are floating. Floating bits take on every possible combination of values; with two floating bits, four actual memory addresses are written:
//
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
// 000000000000000000000000000000111010  (decimal 58)
// 000000000000000000000000000000111011  (decimal 59)
//
// Next, the program is about to write to memory address 26 with a different bitmask:
//
// address: 000000000000000000000000000000011010  (decimal 26)
// mask:    00000000000000000000000000000000X0XX
// result:  00000000000000000000000000000001X0XX
//
// This results in an address with three floating bits, causing writes to eight memory addresses:
//
// 000000000000000000000000000000010000  (decimal 16)
// 000000000000000000000000000000010001  (decimal 17)
// 000000000000000000000000000000010010  (decimal 18)
// 000000000000000000000000000000010011  (decimal 19)
// 000000000000000000000000000000011000  (decimal 24)
// 000000000000000000000000000000011001  (decimal 25)
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
//
// The entire 36-bit address space still begins initialized to the value 0 at every address, and you still need the sum of all values left in memory at the end of the program. In this example, the sum is 208.
//
// Execute the initialization program using an emulator for a version 2 decoder chip. What is the sum of all values left in memory after it completes?
//

use anyhow::{Context, Result, anyhow};
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

const MASK_REGEX: &'static str = r"mask = ([01X]{36})";
const MEM_REGEX: &'static str = r"mem\[([0-9]+)\] = ([0-9]+)";

#[derive(Debug, Default)]
struct Mask {
    mask: String,
}

impl Mask {
    fn new(value: &str) -> Self {
        Mask {
            mask: value.to_owned(),
        }
    }

    fn apply(&self, address: &str) -> Vec<char> {
        let mut resulting_address: [char; 36] = ['0'; 36];
        for i in (0..36).rev() {
            let a = address.chars().nth(i);
            let m = self.mask.chars().nth(i);
            match (a, m) {
                (Some(a), Some(m)) => {
                    // If the bitmask bit is 0, the memory address bit is **unchanged**
                    if m == '0' {
                        resulting_address[i] = a;
                        // If the bitmask bit is 1, the memory address bit is **overwritten with 1**
                    } else if m == '1' {
                        resulting_address[i] = '1';
                        // If the bitmask bit is X, the memory address bit is **floating**
                    } else if m == 'X' {
                        resulting_address[i] = 'X';
                    }
                }
                (_, _) => panic!(
                    "this shouldn't be possible: i: {:?}, address_chars.nth(i) = {:?}, mask_chars.nth(i) = {:?}",
                    i, a, m
                ),
            }
        }
        resulting_address.to_vec()
    }
}

fn generate_addresses(address_template: &Vec<char>) -> Result<Vec<String>> {
    // Generate all the rest of the addresses
    let mut addresses: Vec<String> = Vec::new();
    let digits: usize = address_template.iter().filter(|&&x| x == 'X').count();
    for i in 0..2isize.pow(digits as u32) {
        let mut binary = convert_usize_to_binary(i);
        let zeroes = "0".repeat(digits - binary.len());
        binary.insert_str(0, &zeroes);
        let mut binary_iter = binary.chars();
        let mut new_address = String::new();
        for &char in address_template.iter() {
            if char == 'X' {
                new_address.push(binary_iter.next().unwrap());
            } else {
                new_address.push(char);
            }
        }
        addresses.push(new_address);
    }

    Ok(addresses)
}

fn convert_usize_to_binary(address: isize) -> String {
    let mut res = String::new();
    let mut value = address;
    while value > 1 {
        let residue = value % 2;
        res.push(char::from_digit(residue as u32, 10).unwrap());
        value = value / 2;
    }
    res.push(char::from_digit(value as u32, 10).unwrap());
    res.chars().rev().collect()
}

fn convert_str_to_binary_36bit_str(address: &str) -> Result<String> {
    let address_usize = address
        .parse::<isize>()
        .context("couldn't parse string to usize")?;
    let mut address_binary = convert_usize_to_binary(address_usize);
    let remaining_zeros = 36 - address_binary.len();
    let zeroes = "0".repeat(remaining_zeros);
    address_binary.insert_str(0, &zeroes);
    Ok(address_binary)
}

fn main() -> Result<()> {
    // Read all the input
    let mut buffer = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .context("couldn't read file due to invalid UTF-8")?;
    let lines: Vec<&str> = buffer.trim_end().split("\n").collect();

    // Create regex of mask and memory lines
    let mask_re = Regex::new(MASK_REGEX).context("invalid regex pattern for mask")?;
    let mem_re = Regex::new(MEM_REGEX).context("invalid regex pattern for memory")?;

    let mut mask: Mask = Default::default();
    let mut memory: HashMap<String, u64> = HashMap::new();

    for line in lines {
        match mask_re.captures(line) {
            Some(mask_captures) => {
                mask = match mask_captures.get(1) {
                    Some(mask_value) => Mask::new(mask_value.as_str()),
                    None => Err(anyhow!("mask value wasn't captured using the regex"))?,
                }
            }
            // If it's not mask, it's an mem instruction
            None => match mem_re.captures(line) {
                // Check if it's a valid mem instruction
                Some(mem_captures) => {
                    if mem_captures.len() != 3 {
                        Err(anyhow!(format!(
                            "couldn't capture 2 groups on the mem lines using the regex: {}",
                            line
                        )))?;
                    }

                    // Create all the addresses
                    let orig_address = mem_captures.get(1).unwrap().as_str();
                    let address_binary = convert_str_to_binary_36bit_str(orig_address)?;
                    let resulting_address = mask.apply(&address_binary);
                    let addresses = generate_addresses(&resulting_address)?;

                    // Set all addresses to the value
                    let value = mem_captures
                        .get(2)
                        .unwrap()
                        .as_str()
                        .parse()
                        .context("couldn't parse to u64")?;

                    for address in addresses {
                        memory.insert(address, value);
                    }
                }
                // Not a valid mem instruction either
                None => {
                    Err(anyhow!(format!(
                        "couldn't match/capture neither using the mask regex nor with the mem regex: {}",
                        line
                    )))?;
                }
            },
        }
    }
    println!("Sum is: {}", memory.values().sum::<u64>());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_usize_to_binary() {
        let input = 13197 as isize;
        let actual = convert_usize_to_binary(input);
        let expected = "11001110001101";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_convert_str_to_binary_36bit_str() {
        let input = "13197";
        let actual = convert_str_to_binary_36bit_str(input).unwrap_or_default();
        let expected = "000000000000000000000011001110001101";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mask_apply() {
        let mask_value = "00X10101X110010011XX0X011X100000X010";
        let mask = Mask::new(mask_value);
        let address_binary = "000000000000000000000011001110001101"; // this is in decimal 13197
        let actual: String = mask.apply(address_binary).into_iter().collect();
        let expected = "00X10101X110010011XX0X111X111000X111";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mask_apply_2() {
        let mask_value = "00000000000000000000000000000000X0XX";
        let mask = Mask::new(mask_value);
        let address_binary = "000000000000000000000000000000011010"; // 26
        let actual: String = mask.apply(address_binary).into_iter().collect();
        let expected = "00000000000000000000000000000001X0XX";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_generate_addresses() {
        let input: Vec<char> = "00000000000000000000000000000001X0XX".chars().collect();
        let actual = generate_addresses(&input).unwrap();
        let expected = vec![
            "000000000000000000000000000000010000", // 16
            "000000000000000000000000000000010001", // 17
            "000000000000000000000000000000010010", // 18
            "000000000000000000000000000000010011", // 19
            "000000000000000000000000000000011000", // 24
            "000000000000000000000000000000011001", // 25
            "000000000000000000000000000000011010", // 26
            "000000000000000000000000000000011011", // 27
        ];
        assert_eq!(actual, expected);
    }
}
