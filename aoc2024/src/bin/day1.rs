use std::collections::HashMap;

use aoc::{fetch_input, lines};

fn main() {
    let input: Vec<_> = lines(fetch_input(2024, 1))
        .into_iter()
        .map(|line| {
            let (a, b) = line.split_once(" ").unwrap();
            let a: usize = a.trim().parse().unwrap();
            let b: usize = b.trim().parse().unwrap();
            (a, b)
        })
        .collect();

    let mut lefts: Vec<_> = input.iter().map(|t| t.0).collect();
    let mut rights: Vec<_> = input.iter().map(|t| t.1).collect();

    lefts.sort();
    rights.sort();

    let part1: usize = lefts.iter().zip(&rights).map(|(a, b)| a.abs_diff(*b)).sum();

    println!("part1 = {part1}");

    // Need number of times a given number appears in the rights.
    let mut right_counts: HashMap<usize, usize> = HashMap::new();
    for right in rights {
        *right_counts.entry(right).or_default() += 1;
    }

    let mut part2 = 0;
    for left in lefts {
        let count = right_counts.get(&left);
        if let Some(count) = count {
            part2 += count * left;
        }
    }

    println!("part2 = {part2}");
}
