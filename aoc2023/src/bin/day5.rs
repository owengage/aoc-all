use std::collections::HashSet;

use aoc::line_blocks;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = line_blocks("input/work5");
    let seeds = parse_seeds(&input[0][0]);
    let seeds_part2 = parse_seeds_part2(&input[0][0]);
    let maps: Vec<RangeMap> = input[1..].iter().map(|b| parse_map(b)).collect();

    let part1 = part1(&seeds, &maps);
    let part2 = part2(&seeds_part2, &maps);
    dbg!(part1);
    dbg!(part2);
}

fn part2(seeds: &[(usize, usize)], maps: &[RangeMap]) -> usize {
    // Maps happen to be in order, so just go through them.

    let mins: Vec<_> = seeds
        .par_iter()
        .map(|&(seed_start, seed_len)| {
            let mut min = usize::MAX;
            println!("Starting range: {}..{}", seed_start, seed_start + seed_len);
            for seed in seed_start..(seed_start + seed_len) {
                let mut id = seed;
                let mut current_type = "seed";
                for map in maps {
                    assert_eq!(map.source, current_type);
                    id = map.get(id);
                    current_type = &map.dest;
                }
                assert_eq!("location", current_type);

                min = min.min(id);
            }
            println!("Finished range: {}..{}", seed_start, seed_start + seed_len);
            min
        })
        .collect();

    mins.into_iter().min().unwrap()
}

fn parse_seeds_part2(input: &str) -> Vec<(usize, usize)> {
    let (tag, seeds) = input.split_once(": ").unwrap();
    assert_eq!(tag, "seeds");
    let pairs: Vec<usize> = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    pairs.chunks(2).map(|pair| (pair[0], pair[1])).collect()
}

fn part1(seeds: &HashSet<usize>, maps: &[RangeMap]) -> usize {
    let mut min = usize::MAX;

    // Maps happen to be in order, so just go through them.
    for seed in seeds {
        let mut id = *seed;
        let mut current_type = "seed";
        for map in maps {
            assert_eq!(map.source, current_type);
            id = map.get(id);
            current_type = &map.dest;
        }
        assert_eq!("location", current_type);

        min = min.min(id);
    }

    min
}

fn parse_seeds(input: &str) -> HashSet<usize> {
    let (tag, seeds) = input.split_once(": ").unwrap();
    assert_eq!(tag, "seeds");
    seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct Range {
    source_start: usize,
    dest_start: usize,
    len: usize,
}

fn parse_map(input: &[String]) -> RangeMap {
    let info = &input[0];
    let info = info.strip_suffix(" map:").unwrap();
    let (source, dest) = info.split_once("-to-").unwrap();
    let mut ranges = vec![];
    for line in &input[1..] {
        let ns: Vec<usize> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        assert_eq!(ns.len(), 3);
        ranges.push(Range {
            source_start: ns[1],
            dest_start: ns[0],
            len: ns[2],
        })
    }

    RangeMap::new(source, dest, ranges)
}

#[derive(Debug)]
struct RangeMap {
    source: String,
    dest: String,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Pos {
    i: usize,
}

impl RangeMap {
    fn new(source: &str, dest: &str, ranges: Vec<Range>) -> RangeMap {
        let mut points = vec![];
        for r in &ranges {
            points.push(Pos { i: r.source_start });
            points.push(Pos {
                i: r.source_start + r.len,
            });
        }

        points.sort_by_key(|p| p.i);

        RangeMap {
            source: source.to_string(),
            dest: dest.to_string(),
            ranges,
        }
    }

    fn get(&self, id: usize) -> usize {
        for range in &self.ranges {
            if id >= range.source_start && id < range.source_start + range.len {
                let dist_into_source = id - range.source_start;
                return range.dest_start + dist_into_source;
            }
        }

        id
    }
}
