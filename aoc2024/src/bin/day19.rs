use std::collections::HashMap;

use aoc::{fetch_input, line_blocks};
use itertools::Itertools;

fn main() {
    let input = line_blocks(fetch_input(2024, 19));
    let towels = input[0][0].split(", ").collect_vec();
    let patterns = &input[1];
    let mut cache = HashMap::<&str, usize>::new();

    let mut part1 = 0;
    let mut part2 = 0;

    for p in patterns {
        let designs = find_design(&mut cache, &towels, p);
        part2 += designs;
        if designs > 0 {
            part1 += 1;
        }
    }

    println!("part1 = {part1}");
    println!("part2 = {part2}");
}

fn find_design<'inp>(
    cache: &mut HashMap<&'inp str, usize>,
    towels: &'inp [&str],
    pattern: &'inp str,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(ans) = cache.get(pattern) {
        return *ans;
    }

    let mut matches = 0;

    for towel in towels {
        if let Some(stripped) = pattern.strip_prefix(towel) {
            matches += find_design(cache, towels, stripped);
        }
    }

    cache.insert(pattern, matches);
    matches
}
