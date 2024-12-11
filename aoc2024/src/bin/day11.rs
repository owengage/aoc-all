use std::collections::HashMap;

use aoc::{fetch_input, text, StrExt};
use itertools::Itertools;

fn main() {
    let input = text(fetch_input(2024, 11));
    let stones: Vec<isize> = input.as_str().trim().split_parse(" ").collect_vec();
    let mut counts = stones.into_iter().counts();

    (0..25).for_each(|_| blink(&mut counts));
    println!("part1 = {:?}", counts.values().sum::<usize>());

    (0..50).for_each(|_| blink(&mut counts));
    println!("part2 = {:?}", counts.values().sum::<usize>());
}

fn blink(counts: &mut HashMap<isize, usize>) {
    let mut new = HashMap::new();

    for (stone, count) in counts.iter() {
        if *stone == 0 {
            *new.entry(1).or_default() += count;
            continue;
        }

        let s = stone.to_string();
        if s.len() % 2 == 0 {
            let (a, b) = s.split_at(s.len() / 2);
            *new.entry(a.parse().unwrap()).or_default() += count;
            *new.entry(b.parse().unwrap()).or_default() += count;
            continue;
        }

        *new.entry(stone * 2024).or_default() += count;
    }

    *counts = new;
}
