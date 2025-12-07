use std::ops::RangeInclusive;

use aoc::{fetch_input, lines};

fn parse_inc_range(r: &str) -> RangeInclusive<u32> {
    let (s, e) = r.split_once('-').unwrap();
    let s: u32 = s.parse().unwrap();
    let e: u32 = e.parse().unwrap();
    s..=e
}

fn main() {
    let input = lines(fetch_input(2022, 4));

    let parsed: Vec<_> = input
        .into_iter()
        .flat_map(|line| {
            let (p1, p2) = line.split_once(',').unwrap();
            [parse_inc_range(p1), parse_inc_range(p2)]
        })
        .collect();

    let part1 = parsed
        .chunks(2)
        .map(|rs| {
            let r1 = &rs[0];
            let r2 = &rs[1];
            (r1.contains(r2.start()) && r1.contains(r2.end()))
                || (r2.contains(r1.start()) && r2.contains(r1.end()))
        })
        .map(|b| b as u32)
        .sum::<u32>();

    let part2 = parsed
        .chunks(2)
        .map(|rs| {
            let r1 = &rs[0];
            let r2 = &rs[1];
            (r1.contains(r2.start()) || r1.contains(r2.end()))
                || (r2.contains(r1.start()) || r2.contains(r1.end()))
        })
        .map(|b| b as u32)
        .sum::<u32>();

    dbg!(part1);
    dbg!(part2);
}
