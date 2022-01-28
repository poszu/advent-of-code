use std::io::prelude::*;
use std::{collections::HashSet, io};

use day8_2020::parser::{parse_instruction, Instruction};

/// Add a signed value to an unsigned one
fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as usize
    } else {
        u + i as usize
    }
}

fn parse_instructions<S: AsRef<str>>(input: impl IntoIterator<Item = S>) -> Vec<Instruction> {
    input
        .into_iter()
        .map(|line| {
            let (_, instruction) = parse_instruction(line.as_ref()).unwrap();
            instruction
        })
        .collect()
}

/// Accumulator
type Acc = i32;
/// Intruction Pointer
type IP = usize;
/// History of executed instructions
type IHistory = Vec<Instruction>;

#[derive(PartialEq, Debug)]
enum Execution {
    InfLoop(IP, Acc, IHistory),
    Finished(Acc),
}

/// Find the execution till hitting an infitite loop.
fn execute(mut ip: usize, mut acc: i32, program: &[Instruction]) -> Execution {
    let mut executed_instructions = HashSet::new();
    let mut instruction_history = IHistory::new();

    loop {
        if !executed_instructions.insert(ip) {
            break;
        }

        if ip >= program.len() {
            return Execution::Finished(acc);
        }

        instruction_history.push(program[ip]);

        match program[ip] {
            Instruction::Nop(_) => ip += 1,
            Instruction::Acc(arg) => {
                acc += arg;
                ip += 1
            }
            Instruction::Jmp(arg) => ip = add(ip, arg),
        }
    }
    Execution::InfLoop(ip, acc, instruction_history)
}

/// Fix the program
/// If successfull, returns IP at which program needs fixing and accumulator after the fixed program finished.
fn try_fix_program(
    mut ip: usize,
    mut acc: i32,
    program: &mut [Instruction],
    mut history: &[Instruction],
) -> Result<(IP, Acc), &'static str> {
    loop {
        match history.last() {
            Some(Instruction::Jmp(jmp)) => ip = add(ip, -jmp),
            Some(Instruction::Acc(_) | Instruction::Nop(_)) => ip = add(ip, -1),
            _ => return Err("Ran out of instructions in the history"),
        };
        history = &history[0..add(history.len(), -1)];
        if let Instruction::Acc(val) = program[ip] {
            acc -= val;
        }
        let program_at_ip = program[ip];
        match program_at_ip {
            Instruction::Nop(val) => program[ip] = Instruction::Jmp(val),
            Instruction::Jmp(val) => program[ip] = Instruction::Nop(val),
            _ => continue, // Nothing changed, continue to previous instruction
        }
        if let Execution::Finished(acc) = execute(ip, acc, program) {
            return Ok((ip, acc));
        }
        program[ip] = program_at_ip;
    }
}

fn main() {
    let mut program = parse_instructions(io::stdin().lock().lines().map(|l| l.unwrap()));
    if let Execution::InfLoop(ip, acc, history) = execute(0, 0, &program) {
        println!("PART 1: Acc = {}", acc);
        if let Ok((_, acc)) = try_fix_program(ip, acc, &mut program, &history) {
            println!("PART 2: Acc = {}", acc);
        } else {
            panic!("Failed to fix the program")
        }
    } else {
        panic!("The program was supposed to enter an infinite loop");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_acc() {
        let test_data = [
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ];
        let mut program = parse_instructions(test_data);
        let res = execute(0, 0, &program);

        if let Execution::InfLoop(ip, acc, history) = res {
            assert_eq!(try_fix_program(ip, acc, &mut program, &history), Ok((7, 8)));
        } else {
            panic!("It was supposed to be an infinite loop")
        }
    }
}
