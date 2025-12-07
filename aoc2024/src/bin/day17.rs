use core::panic;

use aoc::{fetch_input, line_blocks, StrExt};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Registers {
    a: isize,
    b: isize,
    c: isize,
}

fn main() {
    let input = line_blocks(fetch_input(2024, 17));
    let registers = parse_registers(&input[0]);
    let instructions = parse_instructions(&input[1]);

    let output = simulate(registers, &instructions);
    let part1 = output.into_iter().map(|v| v.to_string()).join(",");

    let mut candidates = vec![];

    for a in 0..2isize.pow(10) {
        if simulate_verify(Registers { a, b: 0, c: 0 }, &instructions, 1) {
            candidates.push(a);
        }
    }

    for out in 0..instructions.len() - 2 {
        let mut next_candidates = vec![];

        for candidate in candidates {
            for top3 in 0..8 {
                let a = (top3 << (10 + out * 3)) + candidate;
                if simulate_verify(Registers { a, b: 0, c: 0 }, &instructions, out + 2) {
                    next_candidates.push(a);
                }
            }
        }

        candidates = next_candidates;
    }

    println!("part1 = {part1}");
    println!("part2 = {}", candidates[0]);
}

fn simulate(mut regs: Registers, instructions: &[u8]) -> Vec<isize> {
    let mut ip = 0;
    let mut output = vec![];

    while let Some(inst) = instructions.get(ip) {
        let operand = instructions[ip + 1];

        match inst {
            0 => {
                // adv
                let numer = regs.a;
                let denom = 2isize.pow(combo(&regs, operand).try_into().unwrap());
                regs.a = numer / denom;
                ip += 2;
            }
            1 => {
                // bxl
                // Worried about sign? operand < 8 so never touches the sign bit.
                regs.b ^= operand as isize;
                ip += 2;
            }
            2 => {
                // bst
                regs.b = combo(&regs, operand) % 8;
                ip += 2;
            }
            3 => {
                // jnz
                if regs.a == 0 {
                    // do nothing
                    ip += 2;
                } else {
                    ip = operand as usize;
                }
            }
            4 => {
                // bxc
                regs.b ^= regs.c;
                ip += 2;
            }
            5 => {
                // out
                output.push(combo(&regs, operand) % 8);
                ip += 2;
            }
            6 => {
                // bdv
                let numer = regs.a;
                let denom = 2isize.pow(combo(&regs, operand).try_into().unwrap());
                regs.b = numer / denom;
                ip += 2;
            }
            7 => {
                // cdv
                let numer = regs.a;
                let denom = 2isize.pow(combo(&regs, operand).try_into().unwrap());
                regs.c = numer / denom;
                ip += 2;
            }
            _ => panic!("invalid instruction"),
        }
    }

    output
}

fn simulate_verify(mut regs: Registers, instructions: &[u8], upto: usize) -> bool {
    let mut ip = 0;
    let mut output_index = 0;

    while let Some(inst) = instructions.get(ip) {
        let operand = instructions[ip + 1];

        match inst {
            0 => {
                // adv
                let numer = regs.a;
                let denom = 2isize.pow(combo(&regs, operand).try_into().unwrap());
                regs.a = numer / denom;
                ip += 2;
            }
            1 => {
                // bxl
                // Worried about sign? operand < 8 so never touches the sign bit.
                regs.b ^= operand as isize;
                ip += 2;
            }
            2 => {
                // bst
                regs.b = combo(&regs, operand) % 8;
                ip += 2;
            }
            3 => {
                // jnz
                if regs.a == 0 {
                    // do nothing
                    ip += 2;
                } else {
                    ip = operand as usize;
                }
            }
            4 => {
                // bxc
                regs.b ^= regs.c;
                ip += 2;
            }
            5 => {
                // out
                let actual = combo(&regs, operand) % 8;
                let expected = instructions[output_index];
                output_index += 1;

                if expected as isize != actual {
                    return false;
                }
                if output_index == upto {
                    return true;
                }
                ip += 2;
            }
            6 => {
                // bdv
                let numer = regs.a;
                let denom = 2isize.pow(combo(&regs, operand).try_into().unwrap());
                regs.b = numer / denom;
                ip += 2;
            }
            7 => {
                // cdv
                let numer = regs.a;
                let denom = 2isize.pow(combo(&regs, operand).try_into().unwrap());
                regs.c = numer / denom;
                ip += 2;
            }
            _ => panic!("invalid instruction"),
        }
    }

    false
}

fn combo(r: &Registers, operand: u8) -> isize {
    match operand {
        0..=3 => operand as isize,
        4 => r.a,
        5 => r.b,
        6 => r.c,
        7 => panic!("combo 7"),
        _ => panic!("invalid operand"),
    }
}

fn parse_instructions(input: &[String]) -> Vec<u8> {
    input[0]
        .strip_prefix("Program: ")
        .unwrap()
        .split_parse(",")
        .collect()
}

fn parse_registers(inputs: &[String]) -> Registers {
    let p = |line: &String| line.split_once(": ").unwrap().1.parse().unwrap();
    Registers {
        a: p(&inputs[0]),
        b: p(&inputs[1]),
        c: p(&inputs[2]),
    }
}
