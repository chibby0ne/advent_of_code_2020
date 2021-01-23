/*
After some careful analysis, you believe that exactly one instruction is corrupted.

Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp.
(No acc instructions were harmed in the corruption of this boot code.)

The program is supposed to terminate by attempting to execute an instruction immediately after the
last instruction in the file. By changing exactly one jmp or nop, you can repair the boot code and
make it terminate correctly.

For example, consider the same program from above:

nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6

If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction
infinite loop, never leaving that instruction. If you change almost any of the jmp instructions,
the program will still eventually find another jmp instruction and loop forever.

However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program
terminates! The instructions are visited in this order:

nop +0  | 1
acc +1  | 2
jmp +4  | 3
acc +3  |
jmp -3  |
acc -99 |
acc +1  | 4
nop -4  | 5
acc +6  | 6

After the last instruction (acc +6), the program terminates by attempting to run the instruction
below the last instruction in the file. With this change, after the program terminates, the
accumulator contains the value 8 (acc +1, acc +1, acc +6).

Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to
jmp). What is the value of the accumulator after the program terminates?
*/

use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[derive(Debug,Clone,Copy)]
enum InstructionType {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

fn convert_to_instruction_type(line: &str) -> Option<InstructionType> {
    let mut iter = line.split_whitespace();
    let instruction_type = iter.next();
    let operand = iter.next();
    match (instruction_type, operand) {
        (Some(instruction), Some(val)) => match (instruction, val.parse::<i64>()) {
            ("acc", Ok(arg)) => Some(InstructionType::Acc(arg)),
            ("jmp", Ok(arg)) => Some(InstructionType::Jmp(arg)),
            ("nop", Ok(arg)) => Some(InstructionType::Nop(arg)),
            (_, _) => None,
        },
        (_, _) => None,
    }
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let instructions: Vec<InstructionType>  = stdin.lock()
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| convert_to_instruction_type(&x))
        .collect();

    let mut accumulator: i64 = 0;
    let mut instructions_numbers_processed: HashSet<usize> = HashSet::new();
    let mut instructions_numbers_processed_vec: Vec<usize> = Vec::new();
    let mut instructions_processed: Vec<InstructionType> = Vec::new();
    let mut instruction_number: usize = 0;

    // Find program loop and break from it
    loop {
        if let Some(_) = instructions_numbers_processed.get(&instruction_number) {
            break;
        }
        instructions_numbers_processed.insert(instruction_number);
        instructions_numbers_processed_vec.push(instruction_number);
        match instructions.get(instruction_number) {
            Some(ins) => {
                match ins {
                    InstructionType::Nop(_) => {
                        instruction_number += 1;
                    }
                    InstructionType::Acc(val) => {
                        accumulator += val;
                        instruction_number += 1;
                    }
                    InstructionType::Jmp(val) => {
                        instruction_number = (instruction_number as i64 + *val) as usize;
                    }
                };
                instructions_processed.push(*ins);
            },
            _ => panic!("No instructions found at instruction number: {}", instruction_number),
        }
    }

    // Revisit the instructions from the last one to the first one
    // Evaluate if a nop can be replaced by a jmp or a jmp can be replaced by a nop in order to end
    // the program
    assert_eq!(instructions_numbers_processed_vec.len(), instructions_processed.len());
    let mut new_accumulator = accumulator;
    let mut changed_instr;
    for (candidate_instr, candidate_instr_number) in  instructions_processed.iter().zip(instructions_numbers_processed_vec.iter()).rev() {
        match candidate_instr {
            InstructionType::Nop(val) => changed_instr = InstructionType::Jmp(*val),
            InstructionType::Jmp(val) => changed_instr = InstructionType::Nop(*val),
            InstructionType::Acc(val) => {
                new_accumulator = accumulator - *val;
                continue;
            },
        };
        dbg!(candidate_instr_number);
        dbg!(new_accumulator);

        let (stop, final_acc) = run_modified_program(&instructions, *candidate_instr_number, &changed_instr, new_accumulator);
        if stop {
            accumulator = final_acc;
            break;
        }
    }
    println!("Accumulator is: {}", accumulator);
    Ok(())
}

fn run_modified_program(instructions: &Vec<InstructionType>, start_instruction_num: usize, start_instruction: &InstructionType, accumulator: i64) -> (bool, i64) {
    let mut instruction_number = match start_instruction {
        InstructionType::Nop(_) => start_instruction_num + 1,
        InstructionType::Jmp(val) => {
            dbg!(start_instruction_num as i64 + val);
            (start_instruction_num as i64 + val) as usize
        },
        _ => panic!("found an instruction that is not of type jmp or nop"),
    };
    let mut instructions_numbers_processed: HashSet<usize> = HashSet::new();
    let mut instructions_numbers_processed_vec: Vec<usize> = Vec::new();
    let mut new_accumulator = dbg!(accumulator);
    loop {
        if let Some(_) = instructions_numbers_processed.get(&instruction_number) {
            dbg!("returning because of a found loop");
            return (false, new_accumulator)
        }
        instructions_numbers_processed.insert(instruction_number);
        instructions_numbers_processed_vec.push(instruction_number);
        match instructions.get(instruction_number) {
            Some(ins) => {
                match ins {
                    InstructionType::Nop(_) => {
                        instruction_number += 1;
                    }
                    InstructionType::Acc(val) => {
                        new_accumulator += val;
                        instruction_number += 1;
                    }
                    InstructionType::Jmp(val) => {
                        instruction_number = (instruction_number as i64 + *val) as usize;
                    }
                };
            },
            _ => {
                dbg!("returning because of found a non loop");
                dbg!(instructions_numbers_processed_vec);
                return (true, new_accumulator)
            }
        }
    }
}
