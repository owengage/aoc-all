use aoc::{StrExt, fetch_input, line_blocks, two::DenseField};
use itertools::Itertools;

struct Problem {
    area: DenseField<Cell>,
    unplaced: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Present,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Cell::Empty,
            b'#' => Cell::Present,
            _ => panic!(),
        }
    }
}

fn main() {
    let input = line_blocks(fetch_input(2025, 12));
    let presents = input[..input.len() - 1]
        .iter()
        .map(|block| DenseField::<Cell>::from_lines(block[1..].to_vec()))
        .collect_vec();

    let problems = input
        .last()
        .unwrap()
        .iter()
        .map(|line| {
            let (size, counts) = line.split_once(": ").unwrap();
            let (x, y) = size.split_once_parse::<isize>("x");
            let counts = counts.split_parse::<usize>(" ").collect_vec();
            Problem {
                area: DenseField::new(x, y, Cell::Empty),
                unplaced: counts,
            }
        })
        .collect_vec();

    let part1 = part1(&presents, &problems);
    dbg!(part1);
    assert_eq!(492, part1);
}

fn part1(presents: &[DenseField<Cell>], problems: &[Problem]) -> usize {
    let mut fits = 0;

    let areas = presents
        .iter()
        .map(|p| p.data().iter().filter(|&c| *c == Cell::Present).count())
        .collect_vec();

    for problem in problems {
        let min_required_area: usize = areas
            .iter()
            .zip(&problem.unplaced)
            .map(|(area, count)| area * count)
            .sum();

        if min_required_area as isize > problem.area.width() * problem.area.height() {
            continue; // simply not possible to fit.
        }

        // Solution is that they all trivially fit so long as they're not
        // obviously impossible...
        fits += 1;
    }

    fits
}
