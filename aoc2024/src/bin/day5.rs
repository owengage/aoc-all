use std::cmp::Ordering;

use aoc::StrExt;
use aoc::{fetch_input, line_blocks};
use itertools::Itertools;

fn main() {
    let input = line_blocks(fetch_input(2024, 5));
    let rules = parse_rules(&input[0]);
    let updates = parse_updates(&input[1]);

    let (ordered, unordered): (Vec<_>, _) = updates
        .into_iter()
        .partition(|u| *u == to_ordered(&rules, u.to_vec()));

    let reordered = unordered
        .into_iter()
        .map(|up| to_ordered(&rules, up))
        .collect_vec();

    println!("part1 = {}", midsum(ordered));
    println!("part2 = {}", midsum(reordered));
}

fn midsum(updates: Vec<Vec<isize>>) -> isize {
    updates.iter().map(|update| update[update.len() / 2]).sum()
}

fn to_ordered(rules: &[Vec<isize>; 100], mut update: Vec<isize>) -> Vec<isize> {
    update.sort_by(|a, b| {
        if rules[*a as usize].contains(b) {
            Ordering::Less
        } else if rules[*b as usize].contains(a) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    update
}

fn parse_updates(updates: &[String]) -> Vec<Vec<isize>> {
    updates
        .iter()
        .map(|line| line.as_str().split_parse(",").collect_vec())
        .collect_vec()
}

fn parse_rules(rules: &[String]) -> [Vec<isize>; 100] {
    let mut ret = [const { vec![] }; 100];
    for rule in rules {
        let (l, r) = rule.as_str().split_once_parse("|");
        ret[l as usize].push(r);
    }
    ret
}
