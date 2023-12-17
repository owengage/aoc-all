use core::panic;
use std::collections::{HashSet, VecDeque};

use aoc::{
    lines,
    two::{pt, DenseField, Dirn, Point, DOWN, LEFT, RIGHT, UP},
};

#[derive(Debug, Clone, Copy)]
enum Cell {
    LeftDown,
    LeftUp,
    SplitVert,
    SplitHorz,
    Empty,
}

fn main() {
    // let input = lines("input/day16");
    let input = lines("aoc2023/input/day16");
    let field = DenseField::<Cell>::from_lines(input);

    // We probably only care about if a laser goes through a point, and in which
    // direction. If we hit a point and it already has the hit, we can stop.
    // This avoids us looping forever.

    let part1 = energize(&field, pt(0, 0), Dirn::Right);
    dbg!(part1);

    let part2 = part2(&field);
    dbg!(part2);
}

fn part2(field: &DenseField<Cell>) -> usize {
    let mut max_energize = 0;

    for y in 0..field.height {
        max_energize = max_energize.max(energize(field, pt(0, y), Dirn::Right));
        max_energize = max_energize.max(energize(field, pt(field.width - 1, y), Dirn::Left));
    }

    for x in 0..field.width {
        max_energize = max_energize.max(energize(field, pt(x, 0), Dirn::Down));
        max_energize = max_energize.max(energize(field, pt(x, field.height - 1), Dirn::Up));
    }

    max_energize
}

fn energize(field: &DenseField<Cell>, start: Point<isize>, dirn: Dirn) -> usize {
    let mut q = VecDeque::new();
    let mut seen = HashSet::<(Point<isize>, Dirn)>::new();

    q.push_back((start, dirn));

    while let Some((p, dirn)) = q.pop_back() {
        if field.try_get(p.x, p.y).is_none() {
            continue; // fell off.
        }

        if !seen.insert((p, dirn)) {
            // Skip if we've seen this position and direction before.
            continue;
        }

        let cell = field.get(p.x, p.y);

        match cell {
            Cell::LeftDown => match dirn {
                //  \
                Dirn::Left => q.push_back((p + UP, Dirn::Up)),
                Dirn::Right => q.push_back((p + DOWN, Dirn::Down)),
                Dirn::Up => q.push_back((p + LEFT, Dirn::Left)),
                Dirn::Down => q.push_back((p + RIGHT, Dirn::Right)),
            },
            Cell::LeftUp => match dirn {
                //  /
                Dirn::Left => q.push_back((p + DOWN, Dirn::Down)),
                Dirn::Right => q.push_back((p + UP, Dirn::Up)),
                Dirn::Up => q.push_back((p + RIGHT, Dirn::Right)),
                Dirn::Down => q.push_back((p + LEFT, Dirn::Left)),
            },
            Cell::SplitVert => match dirn {
                Dirn::Right | Dirn::Left => {
                    q.push_back((p + UP, Dirn::Up));
                    q.push_back((p + DOWN, Dirn::Down));
                }
                Dirn::Up => q.push_back((p + UP, Dirn::Up)),
                Dirn::Down => q.push_back((p + DOWN, Dirn::Down)),
            },
            Cell::SplitHorz => match dirn {
                Dirn::Right => q.push_back((p + RIGHT, Dirn::Right)),
                Dirn::Left => q.push_back((p + LEFT, Dirn::Left)),
                Dirn::Up | Dirn::Down => {
                    q.push_back((p + LEFT, Dirn::Left));
                    q.push_back((p + RIGHT, Dirn::Right));
                }
            },
            Cell::Empty => {
                // Just continue on.
                let next_pt = p + match dirn {
                    Dirn::Right => RIGHT,
                    Dirn::Left => LEFT,
                    Dirn::Up => UP,
                    Dirn::Down => DOWN,
                };

                q.push_back((next_pt, dirn));
            }
        }
    }

    let points: HashSet<Point<isize>> = seen.iter().map(|(p, _d)| *p).collect();
    points.len()
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'/' => Cell::LeftUp,
            b'\\' => Cell::LeftDown,
            b'|' => Cell::SplitVert,
            b'-' => Cell::SplitHorz,
            b'.' => Cell::Empty,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_parse() {}
}
