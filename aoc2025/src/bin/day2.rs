use std::collections::HashSet;

use aoc::{StrExt, fetch_input, text};

fn main() {
    let input = text(fetch_input(2025, 2));
    let input: Vec<_> = input
        .split(",")
        .map(|range| range.trim().split_once_parse::<usize>("-"))
        .collect();

    let part1 = part1(&input);
    println!("part1 = {part1}");

    let part2 = part2(&input);
    println!("part2 = {part2}");
    // assert_eq!(31898925685, part2);
}

fn part1(input: &[(usize, usize)]) -> usize {
    let mut sum = 0;

    for &(low, high) in input {
        let min_unit_len = low.to_string().len().div_ceil(2);
        let max_unit_len = high.to_string().len() / 2;

        for unit_len in min_unit_len..=max_unit_len {
            let unit_min = 10usize.pow(unit_len as u32 - 1); // 2 -> 10^1 -> 10
            let unit_max = 10usize.pow(unit_len as u32); // 2 -> 10^2 -> 100 EXCLUSIVE.

            for unit in unit_min..unit_max {
                let id: usize = format!("{}{}", unit, unit).parse().unwrap();
                if id >= low && id <= high {
                    sum += id;
                }
            }
        }
    }

    sum
}

fn part2(input: &[(usize, usize)]) -> usize {
    let mut ids = HashSet::new(); // don't double count IDs, eg 1111 could be 11 twice or 1 four times.

    for &(low, high) in input {
        let min_unit_len = 1;
        let max_unit_len = high.to_string().len() / 2;
        let high_len = high.to_string().len();

        for unit_len in min_unit_len..=max_unit_len {
            let unit_min = 10usize.pow(unit_len as u32 - 1); // 2 -> 10^1 -> 10
            let unit_max = 10usize.pow(unit_len as u32); // 2 -> 10^2 -> 100 EXCLUSIVE.

            for unit in unit_min..unit_max {
                let mut r = 2;

                while unit_len * r <= high_len {
                    let id: usize = unit.to_string().repeat(r).parse().unwrap();

                    if (low..=high).contains(&id) {
                        ids.insert(id);
                    }

                    r += 1;
                }
            }
        }
    }

    ids.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::part1;

    #[test]
    fn p1() {
        assert_eq!(11 + 22, part1(&[(11, 22)]));
        assert_eq!(99, part1(&[(95, 115)]));
        assert_eq!(1010, part1(&[(998, 1012)]));
        assert_eq!(0, part1(&[(1698522, 1698528)]));
        assert_eq!(446446, part1(&[(446443, 446449)]));
    }
}
