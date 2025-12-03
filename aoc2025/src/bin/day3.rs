use aoc::{fetch_input, lines};
use itertools::Itertools;

fn main() {
    let input = lines(fetch_input(2025, 3));
    let banks: Vec<Vec<u32>> = input
        .iter()
        .map(|b| b.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let part1 = joltage(banks.clone(), 2);
    let part2 = joltage(banks, 12);
    println!("part1 = {}", part1);
    println!("part2 = {}", part2);
    // assert_eq!(17074, part1);
    // assert_eq!(169512729575727, part2);
}

fn joltage(banks: Vec<Vec<u32>>, digits: usize) -> usize {
    let mut sum = 0;

    for bank in banks {
        let mut value = 0usize;
        let rev = bank.iter().cloned().rev().collect_vec();
        let mut last_index = rev.len();

        for digit in (0..digits).rev() {
            let index = rev[digit..last_index].iter().position_max().unwrap() + digit;
            value = value * 10 + rev[index] as usize;
            last_index = index;
        }

        sum += value as usize;
    }

    sum
}
