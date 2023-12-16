use core::panic;
use std::collections::HashMap;

use aoc::lines;

fn main() {
    let input = lines("aoc2023/input/day12");

    let arrs: Vec<_> = input.iter().map(|s| parse_line(s)).collect();
    let mut cache = Cache::new();

    let part1: usize = arrs
        .iter()
        .map(|(f, sp)| count_tail_arrangements(f, sp, &mut cache))
        .sum();

    let long_arrs: Vec<_> = arrs
        .into_iter()
        .map(|(f, s)| {
            let s_len = s.len();
            let s: Vec<_> = s.into_iter().cycle().take(s_len * 5).collect();
            let f = {
                let mut all = vec![];
                for _ in 0..5 {
                    all.extend_from_slice(&f);
                    all.push('?');
                }
                all.pop();
                all
            };

            (f, s)
        })
        .collect();

    let mut cache = Cache::new();
    let part2: usize = long_arrs
        .iter()
        .map(|(f, sp)| count_tail_arrangements(f, sp, &mut cache))
        .sum();

    dbg!(part1);
    dbg!(part2);
}

type Cache = HashMap<(Vec<char>, Vec<usize>), usize>;

fn count_tail_arrangements(tail: &[char], spec: &[usize], cache: &mut Cache) -> usize {
    // Are we cached?

    let cache_tail = tail.to_vec();
    let cache_spec = spec.to_vec();

    if let Some(entry) = cache.get(&(cache_tail.clone(), cache_spec.clone())) {
        return *entry;
    }

    let (matches, tail, spec) = skip_known(tail, spec);
    if !matches {
        return 0;
    }

    if tail.is_empty() && spec.is_empty() {
        return 1;
    }

    let mut total = 0;

    // Can we place the next block here?
    let mut tail_block = tail.to_vec();
    let mut tail_dot = tail.to_vec();

    if !spec.is_empty() {
        let placed = place_block(&mut tail_block, spec[0]);

        if placed {
            let Some(new_tail) = tail_block.get(spec[0] + 1..) else {
                // we must have completed the entire tail.
                if matches_spec(&tail_block, spec) {
                    return 1;
                } else {
                    return 0;
                }
            };

            total += count_tail_arrangements(new_tail, &spec[1..], cache);
        }
    }

    // Can we place a dot?
    total += match tail_dot.first() {
        Some('#') => 0,
        Some('.' | '?') => {
            tail_dot[0] = '.';
            count_tail_arrangements(&tail_dot[1..], spec, cache)
        }
        None => 0,
        _ => panic!(),
    };

    // Can we memoise this result?
    cache.insert((cache_tail, cache_spec), total);

    total
}

fn skip_known<'t, 's>(tail: &'t [char], mut spec: &'s [usize]) -> (bool, &'t [char], &'s [usize]) {
    let mut current_block = 0;
    let mut last_complete_block = 0;

    for i in 0..tail.len() {
        match tail[i] {
            '#' => {
                current_block += 1;
            }
            '.' => {
                if current_block != 0 {
                    if spec.is_empty() {
                        return (false, tail, spec);
                    }
                    if spec[0] != current_block {
                        // we don't match the spec!
                        return (false, tail, spec);
                    } else {
                        // Complete this block.
                        // What happens if this is the end of a block, we return
                        // the tail ##.?, we return .? rather than just ?
                        spec = &spec[1..];
                        last_complete_block = i + 1; // include the dot.
                        current_block = 0;
                    }
                }
            }
            '?' => return (true, &tail[last_complete_block..], spec),
            _ => panic!(),
        }
    }

    if current_block != 0 {
        if spec.is_empty() {
            return (false, tail, spec);
        }
        if spec[0] != current_block {
            // we don't match the spec!
            (false, tail, spec)
        } else {
            spec = &spec[1..];
            (true, &tail[tail.len()..], spec)
        }
    } else {
        (true, &tail[last_complete_block..], spec)
    }
}

// Try to place the given block immediately. Return true is successful.
fn place_block(tail: &mut [char], block: usize) -> bool {
    for i in 0..block {
        match tail.get(i) {
            Some('#' | '?') => {
                tail[i] = '#';
            }
            Some('.') => return false,
            None => return false,
            _ => panic!(),
        }
    }

    // Now need to place last dot to finish block.
    match tail.get(block) {
        Some('#') => false,
        Some('.') => true,
        Some('?') => {
            tail[block] = '.';
            true
        }
        None => true,
        _ => panic!(),
    }
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let (field, spec) = line.split_once(' ').unwrap();
    (
        field.chars().collect(),
        spec.split(',').map(|s| s.parse().unwrap()).collect(),
    )
}

fn matches_spec(field: &[char], spec: &[usize]) -> bool {
    let mut block_len = 0;
    let mut blocks = vec![];

    for &spring in field {
        if spring == '#' {
            block_len += 1;
        } else if block_len > 0 {
            blocks.push(block_len);
            block_len = 0;
        }
    }

    if block_len > 0 {
        blocks.push(block_len);
    }

    blocks.eq(spec)
}

#[cfg(test)]
mod test {

    use itertools::Itertools;

    use crate::*;

    #[test]
    fn new_way() {
        let mut cache = Cache::new();
        assert_eq!(
            1,
            count_tail_arrangements(&['?', '.', '#'], &[1, 1], &mut cache)
        );
        assert_eq!(
            1,
            count_tail_arrangements(&['?', '?', '#'], &[1, 1], &mut cache)
        );
        assert_eq!(
            1,
            count_tail_arrangements(&['?', '?', '?'], &[1, 1], &mut cache)
        );
        assert_eq!(
            2,
            count_tail_arrangements(&['?', '?', '?'], &[2], &mut cache)
        );
        assert_eq!(
            2,
            count_tail_arrangements(&['?', '#', '?'], &[2], &mut cache)
        );
        assert_eq!(
            3,
            count_tail_arrangements(&['?', '?', '#', '#', '?', '?'], &[4], &mut cache)
        );

        assert_eq!(
            5,
            count_tail_arrangements(
                &[
                    '?', '?', '?', '?', '?', '?', '?', '#', '#', '?', '?', '?', '?', '?', '#', '?',
                    '#', '?'
                ],
                &[9, 6],
                &mut cache
            )
        );

        assert_eq!(
            4,
            count_tail_arrangements(
                &"????.######..#####.".chars().collect_vec(),
                &[1, 6, 5],
                &mut cache
            )
        );
        assert_eq!(
            10,
            count_tail_arrangements(
                &"?###????????".chars().collect_vec(),
                &[3, 2, 1],
                &mut cache
            )
        );
        assert_eq!(
            1,
            count_tail_arrangements(
                &"?#?#?#?#?#?#?#?".chars().collect_vec(),
                &[1, 3, 1, 6],
                &mut cache
            )
        );
        assert_eq!(
            4,
            count_tail_arrangements(&"??????#.??".chars().collect_vec(), &[2, 2], &mut cache) // ??????#.??
                                                                                              // ##...##...
                                                                                              // .##..##...
                                                                                              // ..##.##...
                                                                                              // .....##.##
        );
    }

    #[test]
    fn ans() {
        let input = lines("input/work12");
        let mut cache = Cache::new();
        let arrs: Vec<_> = input.iter().map(|s| parse_line(s)).collect();
        let ans: usize = arrs
            .iter()
            .map(|(f, sp)| count_tail_arrangements(f, sp, &mut cache))
            .sum();

        assert_eq!(7857, ans);
    }

    #[test]
    fn matcher() {
        assert!(matches_spec(&"..#".chars().collect_vec(), &[1]));
        assert!(matches_spec(&"#.#".chars().collect_vec(), &[1, 1]));
        assert!(matches_spec(&"##.#".chars().collect_vec(), &[2, 1]));
        assert!(matches_spec(&"##.#".chars().collect_vec(), &[2, 1]));
        assert!(matches_spec(
            &"####.....#...###...#..".chars().collect_vec(),
            &[4, 1, 3, 1]
        ));
    }
}
