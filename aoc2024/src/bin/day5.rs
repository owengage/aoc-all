use std::cmp::Ordering;

use aoc::{fetch_input, line_blocks};
use itertools::Itertools;

fn main() {
    let input = line_blocks(fetch_input(2024, 5));
    let rules = parse_rules(&input[0]);
    let updates = parse_updates(&input[1]);

    let mut ordered = vec![];
    let mut unordered = vec![];

    for update in updates {
        if is_ordered(&rules, &update) {
            ordered.push(update);
        } else {
            unordered.push(update);
        }
    }

    let reordered = unordered
        .into_iter()
        .map(|up| to_ordered(&rules, up))
        .collect_vec();

    let part1 = midsum(ordered);
    let part2: isize = midsum(reordered);

    dbg!(part1);
    dbg!(part2);
}

fn midsum(updates: Vec<Vec<isize>>) -> isize {
    updates.iter().map(|update| update[update.len() / 2]).sum()
}

fn to_ordered(rules: &[Vec<isize>; 100], mut update: Vec<isize>) -> Vec<isize> {
    update.sort_by(|a, b| {
        let alesslist = &rules[*a as usize];
        let blesslist = &rules[*b as usize];
        if alesslist.contains(b) {
            Ordering::Less
        } else if blesslist.contains(a) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    update
}

fn is_ordered(rules: &[Vec<isize>; 100], update: &[isize]) -> bool {
    for (i, val) in update.iter().enumerate() {
        for follower in update[i + 1..].iter() {
            if rules[*follower as usize].contains(val) {
                return false;
            }
        }
    }

    true
}

fn parse_updates(updates: &[String]) -> Vec<Vec<isize>> {
    updates
        .iter()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect_vec())
        .collect_vec()
}

fn parse_rules(rules: &[String]) -> [Vec<isize>; 100] {
    let mut ret = [const { vec![] }; 100];
    for rule in rules {
        let (l, r) = rule.split_once("|").unwrap();
        let l: isize = l.parse().unwrap();
        let r: isize = r.parse().unwrap();
        ret[l as usize].push(r);
    }
    ret
}
