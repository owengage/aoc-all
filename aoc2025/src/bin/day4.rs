use std::usize;

use aoc::{fetch_input, lines, two::DenseField};

fn main() {
    let input = lines(fetch_input(2025, 4));
    let field: DenseField<char> = DenseField::from_lines(input);

    let part1 = part1(&field);
    println!("part1 = {}", part1);
    // assert_eq!(1549, part1);

    let part2 = part2(field);
    println!("part2 = {}", part2);
    // assert_eq!(8887, part2);
}

fn part1(field: &DenseField<char>) -> usize {
    let mut count = 0;

    for p in field.points() {
        if *field.get(p) != '@' {
            continue;
        }

        let adj_rolls = field
            .neighbours8_bounded(p)
            .filter(|(c, _)| **c == '@')
            .count();

        if adj_rolls < 4 {
            count += 1;
        }
    }

    count
}

fn part2(mut field: DenseField<char>) -> usize {
    let mut count = 0;

    let mut removed_this_iteration = usize::MAX; // any positive number

    while removed_this_iteration > 0 {
        removed_this_iteration = 0;

        for p in field.points() {
            if *field.get(p) != '@' {
                continue;
            }

            let adj_rolls = field
                .neighbours8_bounded(p)
                .filter(|(c, _)| **c == '@')
                .count();

            if adj_rolls < 4 {
                removed_this_iteration += 1;
                *field.get_mut(p) = '.';
            }
        }

        count += removed_this_iteration;
    }

    count
}
