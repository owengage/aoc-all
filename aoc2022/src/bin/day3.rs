use core::panic;
use std::collections::{HashMap, HashSet};

use aoc::{fetch_input, lines};

fn to_pri(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn part2_chunk(chunk: &[Vec<u32>]) -> u32 {
    // what pri does each line of the chunk have?
    let mut m = HashMap::new();
    for bag in chunk {
        // make sure items are not repeated.
        let bag = bag.iter().copied().collect::<HashSet<u32>>();
        for pri in bag {
            *m.entry(pri).or_insert(0) += 1;
        }
    }

    for entry in m {
        if entry.1 == 3 {
            return entry.0;
        }
    }
    panic!();
}

fn main() {
    let input = lines(fetch_input(2022, 3));

    let pri = input
        .into_iter()
        .map(|line| line.chars().map(to_pri).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // for each line, there's a letter shared between the two halves. Sum up
    // those.
    let part1: u32 = pri
        .iter()
        .map(|v| v.split_at(v.len() / 2))
        .map(|(l, r)| {
            for p in l {
                if r.contains(p) {
                    return *p;
                }
            }
            0
        })
        .sum();

    let part2: u32 = pri.chunks(3).map(part2_chunk).sum();
    dbg!(part1);
    dbg!(part2);
}
