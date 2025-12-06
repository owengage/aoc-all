use core::panic;

use aoc::{fetch_input, lines};
use itertools::Itertools;

fn main() {
    let input = lines(fetch_input(2025, 6));
    let mut numbers = vec![];
    let ops = input
        .last()
        .unwrap()
        .split_whitespace()
        .map(String::from)
        .collect_vec();

    for line in &input[0..input.len() - 1] {
        let row: Vec<usize> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        numbers.push(row);
    }

    let part1 = part1(&numbers, &ops);
    println!("part1 = {}", part1);

    let part2 = part2(&input);
    println!("part2 = {}", part2);
}

fn part2(input: &[String]) -> usize {
    let mut answers = vec![];

    let mut col = 0;
    let mut row = 0;

    let mut current_num = String::new();
    let mut current_nums = vec![];
    let mut current_op = b' ';

    loop {
        if col == input[0].len() {
            // we've got to the end
            let ans = match current_op {
                b'+' => current_nums.iter().sum::<usize>(),
                b'*' => current_nums.iter().product::<usize>(),
                _ => panic!(),
            };

            current_nums.clear();
            answers.push(ans);
            break;
        }

        let char = input[row].as_bytes()[col];

        match char {
            b' ' => {}
            b'0'..=b'9' => {
                current_num.push(char as char);
            }
            b'+' | b'*' => {
                current_op = char;
            }
            _ => panic!("Unknown op '{}'", char as char),
        }

        row += 1;
        if row == input.len() {
            if current_num.is_empty() {
                // we didn't see any number in this entire column, must mean
                // we're moving on to a new problem.
                let ans = match current_op {
                    b'+' => current_nums.iter().sum::<usize>(),
                    b'*' => current_nums.iter().product::<usize>(),
                    _ => panic!(),
                };

                current_nums.clear();
                answers.push(ans);
                col += 1;
                row = 0;
                current_op = b' ';
                continue;
            }

            let n: usize = current_num.parse().unwrap();
            current_num.clear();
            current_nums.push(n);

            // move to next column.
            col += 1;
            row = 0;
        }
    }

    answers.iter().sum()
}

fn part1(numbers: &[Vec<usize>], ops: &[String]) -> usize {
    let mut answers = vec![0; ops.len()];

    // Set initial values based on the operation.
    for (ans, op) in answers.iter_mut().zip(ops) {
        match op.as_ref() {
            "*" => *ans = 1,
            "+" => *ans = 0,
            _ => panic!(),
        }
    }

    for row in numbers {
        for i in 0..row.len() {
            match ops[i].as_ref() {
                "*" => answers[i] *= row[i],
                "+" => answers[i] += row[i],
                _ => panic!(),
            }
        }
    }

    answers.iter().sum()
}
