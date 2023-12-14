use core::panic;
use std::collections::HashMap;

use aoc::line_blocks;

fn main() {
    let input = line_blocks("input/day8");
    let dirs = parse_directions(&input[0][0]);
    let network = parse_network(&input[1]);

    let part1 = part1(&dirs, &network);
    dbg!(part1);

    let part2 = part2(&dirs, &network);
    dbg!(part2);
}

fn part2(dirs: &[Dir], network: &HashMap<Loc, (Loc, Loc)>) -> usize {
    let mut cycles = vec![];

    for start in network.keys() {
        if start.ends_with('A') {
            let cycle = find_cycle(start, dirs, network);
            // Problem is constructed such that the 'runup' to the loop is the
            // same length as from the Z to the start of the loop. This means we
            // can simple use LCM on the cycle lengths.
            assert_eq!(cycle.len, cycle.offset);
            cycles.push(cycle);
        }
    }

    let mut ans = 1;
    for c in cycles {
        ans = num::integer::lcm(ans, c.len);
    }

    ans
}

#[derive(Debug)]
struct Cycle {
    offset: usize,
    len: usize,
}

fn find_cycle(start: &str, dirs: &[Dir], network: &HashMap<Loc, (Loc, Loc)>) -> Cycle {
    let dirs_len = dirs.len();
    let dirs = dirs.iter().cycle();
    let mut offset = 0;

    // node + place in dirs, value is step it was seen.
    let mut seen = HashMap::<(&str, usize), usize>::new();

    let mut current = start;

    for (i, &dir) in dirs.enumerate() {
        let i_dir = i % dirs_len;

        if let Some(&step) = seen.get(&(current, i_dir)) {
            // found loop
            assert_ne!(offset, 0);
            return Cycle {
                offset,
                len: i - step,
            };
        }

        seen.insert((current, i_dir), i);

        if current.ends_with('Z') {
            // Set the offset if we haven't found an end already.
            if offset == 0 {
                offset = i;
            } else {
                panic!("Found other end");
            }
        }

        let choice = network.get(current).unwrap();

        if dir == Dir::Left {
            current = &choice.0;
        } else {
            current = &choice.1;
        }
    }

    unreachable!()
}

fn part1(dirs: &[Dir], network: &HashMap<String, (String, String)>) -> usize {
    let dirs = dirs.iter().cycle();
    let mut count = 0;
    let mut current = "AAA";

    for &dir in dirs {
        let choice = network.get(current).unwrap();

        if dir == Dir::Left {
            current = &choice.0;
        } else {
            current = &choice.1;
        }
        count += 1;

        if current == "ZZZ" {
            break;
        }
    }

    count
}

type Loc = String;

fn parse_network(input: &[String]) -> HashMap<Loc, (Loc, Loc)> {
    input
        .iter()
        .map(|line| {
            let (src, hops) = line.split_once(" = ").unwrap();
            let hops = hops.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
            let (l, r) = hops.split_once(", ").unwrap();
            (src.to_string(), (l.to_string(), r.to_string()))
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Left,
    Right,
}

fn parse_directions(input: &str) -> Vec<Dir> {
    input
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!(),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ans() {
        let input = line_blocks("input/day8");
        let dirs = parse_directions(&input[0][0]);
        let network = parse_network(&input[1]);
        let part2 = part2(&dirs, &network);
        assert_eq!(part2, 9606140307013);
    }
}
