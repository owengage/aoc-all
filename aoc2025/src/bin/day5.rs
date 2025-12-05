use aoc::{StrExt, fetch_input, line_blocks};
use itertools::Itertools;

fn main() {
    let input = line_blocks(fetch_input(2025, 5));
    let ranges = input[0]
        .iter()
        .map(|range| range.trim().split_once_parse::<usize>("-"))
        .collect_vec();
    let ids: Vec<usize> = input[1].iter().map(|id| id.parse().unwrap()).collect();

    let part1 = part1(&ranges, &ids);
    println!("part1 = {part1}");
    // assert_eq!(756, part1);

    let part2 = part2(&ranges);
    println!("part2 = {part2}");
    // assert_eq!(355555479253787, part2);
}

fn part2(ranges: &[(usize, usize)]) -> usize {
    let mut ranges = ranges.to_vec();
    ranges.sort_by_key(|r| r.0); // sort by lower bound.

    let mut new_ranges = vec![];
    let mut current = ranges[0];

    for next in &ranges[1..] {
        if next.0 > current.1 {
            // we're beyond the end of the current range.
            new_ranges.push(current);
            current = *next;
        } else {
            // absorb next range.
            current = (current.0, current.1.max(next.1));
        }
    }

    new_ranges.push(current);
    ranges = new_ranges;

    // Sanity check
    for rs in ranges.windows(2) {
        assert!(rs[0].1 < rs[1].0);
    }

    let mut sum = 0;
    for r in ranges {
        sum += 1 + r.1 - r.0;
    }

    sum
}

fn part1(ranges: &[(usize, usize)], ids: &[usize]) -> usize {
    let mut count = 0;
    'outer: for id in ids {
        for range in ranges {
            if (range.0..=range.1).contains(&id) {
                count += 1;
                continue 'outer;
            }
        }
    }

    count
}
