use core::panic;
use std::{collections::HashSet, iter};

use aoc::{
    fetch_input, lines,
    two::{DenseField, IPoint, pt},
};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Cell {
    Empty,
    Start,
    Splitter,
    Beam(usize),
}

impl From<u8> for Cell {
    fn from(v: u8) -> Self {
        match v {
            b'.' => Cell::Empty,
            b'^' => Cell::Splitter,
            b'S' => Cell::Start,
            b'|' => Cell::Beam(1),
            _ => panic!(),
        }
    }
}

fn main() {
    let input = lines(fetch_input(2025, 7));
    let field = DenseField::<Cell>::from_lines(input);
    let start = field.find(&Cell::Start).unwrap();

    let split_count = part1(field.clone(), start);
    println!("part1 = {split_count}");
    assert_eq!(1553, split_count);

    let part2 = part2(field, start);
    println!("part2 = {part2}");
    assert_eq!(15811946526915, part2);
}

fn part2(mut field: DenseField<Cell>, start: aoc::two::Point<isize>) -> usize {
    *field.get_mut(start) = Cell::Beam(1);
    let mut timelines = 0;

    // Go one row at a time.
    for head in field.points_row_major() {
        let below = head + pt(0, 1);

        let &Cell::Beam(beam) = field.get(head) else {
            continue; // if we're not a beam just move on.
        };

        match field.try_get(below) {
            Some(c) => match c {
                Cell::Empty => {
                    *field.get_mut(below) = Cell::Beam(beam);
                }
                Cell::Splitter => {
                    let mut handle_path = |p| {
                        let c = field.get_mut(p);
                        match c {
                            Cell::Empty => *c = Cell::Beam(beam),
                            Cell::Beam(n) => *c = Cell::Beam(beam + *n),
                            _ => panic!(),
                        }
                    };

                    let left = below + pt(-1, 0);
                    let right = below + pt(1, 0);
                    handle_path(left);
                    handle_path(right);
                }
                Cell::Beam(beam2) => *field.get_mut(below) = Cell::Beam(beam + beam2),
                Cell::Start => panic!(),
            },
            None => timelines += beam,
        }
    }

    timelines
}

fn part1(mut field: DenseField<Cell>, start: aoc::two::Point<isize>) -> i32 {
    let mut heads = HashSet::<IPoint>::from_iter(iter::once(start));
    let mut split_count = 0;

    while let Some(&head) = heads.iter().next() {
        heads.remove(&head);

        let below = head + pt(0, 1);
        match field.try_get(below) {
            Some(c) => match c {
                Cell::Empty => {
                    *field.get_mut(below) = Cell::Beam(1);
                    heads.insert(below);
                }
                Cell::Splitter => {
                    split_count += 1;
                    let left = below + pt(-1, 0);
                    let right = below + pt(1, 0);
                    field.try_get_mut(left).map(|c| {
                        *c = Cell::Beam(1);
                        heads.insert(left);
                    });
                    field.try_get_mut(right).map(|c| {
                        *c = Cell::Beam(1);
                        heads.insert(right);
                    });
                }
                Cell::Beam(_) => {} // Already being done.
                Cell::Start => panic!(),
            },
            None => {} // off the end.
        }
    }
    split_count
}
