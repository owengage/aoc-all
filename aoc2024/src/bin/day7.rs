use std::ops::{Add, Mul};

use aoc::{fetch_input, lines};
use itertools::Itertools;

struct Calibration {
    test: isize,
    values: Vec<isize>,
}

fn main() {
    let input = lines(fetch_input(2024, 7));
    let cals = input
        .into_iter()
        .map(|line| {
            let (test, vals) = line.split_once(": ").unwrap();
            let test: isize = test.trim().parse().unwrap();
            let values = vals
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect_vec();
            Calibration { test, values }
        })
        .collect_vec();

    let part1_ops = [Add::add, Mul::mul];
    let part2_ops = [Add::add, Mul::mul, cat];

    let process = |ops| {
        let mut sum = 0;
        for cal in &cals {
            if recurse(cal, cal.values[0], 1, ops) {
                sum += cal.test;
            }
        }
        sum
    };

    println!("part1 = {}", process(&part1_ops));
    println!("part2 = {}", process(&part2_ops));
}

fn cat(a: isize, b: isize) -> isize {
    // Faster than converting to strings.
    let digits = ((b as f64).log10()) as u32 + 1;
    let shifter = 10isize.pow(digits);
    a * shifter + b
}

fn recurse<Op>(cal: &Calibration, current: isize, next: usize, ops: &[Op]) -> bool
where
    Op: Fn(isize, isize) -> isize,
{
    if next == cal.values.len() {
        current == cal.test
    } else {
        if current > cal.test {
            return false; // assumes ops only every increase the value.
        }
        ops.iter()
            .any(|op| recurse(cal, op(current, cal.values[next]), next + 1, ops))
    }
}
