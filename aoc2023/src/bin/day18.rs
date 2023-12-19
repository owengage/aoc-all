use std::{
    collections::VecDeque,
    fmt::{Display, Write},
};

use aoc::{
    lines,
    two::{pt, DenseField, Dirn, IPoint},
};
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Cell {
    Trench { colour: String },
    Empty { flooded: bool },
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Trench { .. } => f.write_char('#'),
            Cell::Empty { flooded: true } => f.write_char('o'),
            Cell::Empty { flooded: false } => f.write_char('.'),
        }
    }
}
fn main() {
    let input = lines("aoc2023/input/work18");
    let instructions = parse_instructions(input);
    let bounds = find_bounds(&instructions);
    let width = bounds.1.x - bounds.0.x;
    let height = bounds.1.y - bounds.0.y;
    let start = pt(-bounds.0.x + 2, -bounds.0.y + 2); // ?
    let mut field = DenseField::new(width + 4, height + 4, Cell::Empty { flooded: false });

    dig_trench(start, &instructions, &mut field);

    let inside_point = find_inside_point(&field);

    flood(inside_point, &mut field);

    let part1 = part1(&field);
    dbg!(part1);
}

fn part1(field: &DenseField<Cell>) -> usize {
    let mut count = 0;
    for p in field.points() {
        match field.get(p.x, p.y) {
            Cell::Trench { .. } => count += 1,
            Cell::Empty { flooded: true } => count += 1,
            Cell::Empty { flooded: false } => {}
        };
    }

    count
}

fn flood(start: IPoint, field: &mut DenseField<Cell>) {
    let mut q = VecDeque::new();
    q.push_back(start);

    while let Some(p) = q.pop_front() {
        if let Cell::Empty { flooded: false } = field.get_mut(p.x, p.y) {
            *field.get_mut(p.x, p.y) = Cell::Empty { flooded: true };
        } else {
            continue;
        }

        for (next_cell, next_p) in field.neighbours8_bounded(p.x, p.y) {
            if let Cell::Empty { flooded: false } = next_cell {
                q.push_back(next_p);
            }
        }
    }
}

fn find_inside_point(field: &DenseField<Cell>) -> IPoint {
    for y in 0..field.height() {
        let mut trench_count = 0;
        for x in 0..field.width() {
            let cell = field.get(x, y);
            match cell {
                Cell::Trench { .. } => trench_count += 1,
                Cell::Empty { .. } => {
                    if trench_count == 1 {
                        return pt(x, y);
                    }

                    if trench_count > 1 {
                        break;
                    }
                }
            }
        }
    }
    todo!()
}

fn dig_trench(start: IPoint, instructions: &[Instr], field: &mut DenseField<Cell>) {
    let mut current = start;

    for ins in instructions {
        for d in 0..ins.distance {
            let delta = current + ins.dirn * d;
            let cell = field.get_mut(delta.x, delta.y);
            assert!(delta == start || matches!(cell, Cell::Empty { .. }));

            *cell = Cell::Trench {
                colour: ins.colour.clone(),
            };
        }
        current += ins.dirn * ins.distance;
    }
}

fn find_bounds(instructions: &[Instr]) -> (IPoint, IPoint) {
    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bottom = 0;
    let mut current = pt(0, 0);

    for ins in instructions {
        current += ins.dirn * ins.distance;

        left = left.min(current.x);
        right = right.max(current.x);
        top = top.min(current.y);
        bottom = bottom.max(current.y);
    }

    (pt(left, top), pt(right, bottom))
}

#[derive(Debug)]
struct Instr {
    distance: isize,
    dirn: IPoint,
    colour: String,
}

fn parse_instructions(input: Vec<String>) -> Vec<Instr> {
    input
        .into_iter()
        .map(|l| {
            let parts = l.split_whitespace().collect_vec();
            Instr {
                distance: parts[1].parse().unwrap(),
                dirn: Dirn::from_letter(parts[0]).as_point(),
                colour: parts[2]
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .to_string(),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {

    #[test]
    fn test_parse() {}
}
