use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use aoc::{lines, two::DenseField};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Cell {
    RoundRock,
    SquareRock,
    Empty,
}

fn main() {
    let input = lines("input/day14");
    let mut field1 = DenseField::<Cell>::from_lines(input);
    let field2 = field1.clone();

    roll_north(&mut field1);
    let part1 = north_support_load(&field1);
    dbg!(part1);

    let part2 = part2(field2);
    dbg!(part2);
}

const BILLION: usize = 1_000_000_000;

fn part2(mut field: DenseField<Cell>) -> usize {
    let mut seen = HashMap::new();
    let mut cycle = 0;
    let mut jumped = false;
    loop {
        if cycle == BILLION {
            break;
        }

        if let Some(seen_before) = seen.insert(field.clone(), cycle) {
            if !jumped {
                jumped = true;
                println!("Found loop at cycle {cycle}");

                // We've seen this field before!
                // We can now jump close to the final cycle.
                let cycle_len = cycle - seen_before;
                let cycles_left = BILLION - cycle;
                let loops_remaining = cycles_left / cycle_len;
                cycle += loops_remaining * cycle_len + 1;
            } else {
                cycle += 1;
            }
        } else {
            cycle += 1;
        }

        for _ in 0..4 {
            roll_north(&mut field);
            field.rotate_clockwise();
        }
    }

    north_support_load(&field)
}

fn north_support_load(field: &DenseField<Cell>) -> usize {
    let mut load = 0usize;

    for y in 0..field.height {
        for x in 0..field.width {
            if let Cell::RoundRock = field.get(x, y) {
                load += (field.height - y) as usize;
            }
        }
    }

    load
}

fn roll_north(field: &mut DenseField<Cell>) {
    // For each row starting from the north (y=0) move rounded rocks until they
    // hit something, or the top.
    for y in 0..field.height {
        for x in 0..field.width {
            match field.get(x, y) {
                Cell::RoundRock => {
                    // go down from y to 0 here as much as we can.
                    let mut fell_off = true;
                    for i in 0..y {
                        if *field.get(x, y - i - 1) != Cell::Empty {
                            *field.get_mut(x, y) = Cell::Empty;
                            *field.get_mut(x, y - i) = Cell::RoundRock;
                            fell_off = false;
                            break;
                        }
                    }
                    if fell_off {
                        *field.get_mut(x, y) = Cell::Empty;
                        *field.get_mut(x, 0) = Cell::RoundRock;
                    }
                }
                Cell::SquareRock => {}
                Cell::Empty => {}
            }
        }
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Cell::SquareRock,
            b'O' => Cell::RoundRock,
            b'.' => Cell::Empty,
            _ => panic!(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::RoundRock => 'O',
            Cell::SquareRock => '#',
            Cell::Empty => '.',
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = lines("test-input/day14-1");
        let mut field = DenseField::<Cell>::from_lines(input);
        roll_north(&mut field);
    }

    #[test]
    fn test2() {
        let input = lines("input/day14");
        let field = DenseField::<Cell>::from_lines(input);
        assert_eq!(100310, part2(field));
    }
}
