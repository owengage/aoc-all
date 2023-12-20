use core::panic;
use std::collections::HashMap;

use aoc::{
    lines,
    two::{pt, Dirn, IPoint, DOWN, LEFT, RIGHT, UP},
};
use itertools::Itertools;

fn main() {
    // let input = lines("input/work18");
    let input = lines("aoc2023/input/day18");
    let instructions = parse_instructions(input);
    let part1 = sparse_fill(&instructions);
    dbg!(part1);

    let new_instructions = reparse_instructions(&instructions);
    let part2 = sparse_fill(&new_instructions);

    dbg!(part2);
}

fn reparse_instructions(instructions: &[Instr]) -> Vec<Instr> {
    let mut new_ins = vec![];

    for ins in instructions {
        let data = ins.colour.strip_prefix('#').unwrap();
        let dist = isize::from_str_radix(&data[0..5], 16).unwrap();
        let dirn = match data.chars().last().unwrap() {
            '0' => RIGHT,
            '1' => DOWN,
            '2' => LEFT,
            '3' => UP,
            _ => panic!(),
        };

        new_ins.push(Instr {
            distance: dist,
            dirn,
            colour: String::new(),
        })
    }

    new_ins
}

#[allow(clippy::nonminimal_bool)]
fn sparse_fill(instructions: &[Instr]) -> usize {
    let verts = to_verts(instructions);
    let horz = to_horz(instructions);

    let mut filled = 0;

    let (top, bottom) = verts.iter().fold((0, 0), |acc, line| {
        (acc.0.min(line.0.y), acc.1.max(line.0.y + line.1))
    });

    for row in top..bottom + 1 {
        let relevant = verts
            .iter()
            .filter(|line| (line.0.y..line.0.y + line.1 + 1).contains(&row))
            .collect_vec();

        let mut in_trench = false;

        let count: isize = relevant
            .windows(2)
            .map(|chunk| {
                let left = chunk[0].0.x;
                let right = chunk[1].0.x;

                let left_top = chunk[0].0.y;
                let left_bottom = chunk[0].0.y + chunk[0].1;
                let right_top = chunk[1].0.y;
                let right_bottom = chunk[1].0.y + chunk[1].1;

                let left_crosses = left_top != row;

                if left_crosses {
                    in_trench = !in_trench;
                }

                if (left_top == right_top && left_top == row)
                    || (left_top == right_bottom && left_top == row)
                    || (left_bottom == right_top && left_bottom == row)
                    || (left_bottom == right_bottom && left_bottom == row)
                {
                    if let Some(hor) = horz.get(&row) {
                        for l in hor {
                            if left == l.0.x {
                                return right - left;
                            }
                        }
                    }
                }

                if in_trench {
                    right - left
                } else {
                    1
                }
            })
            .sum();

        // We don't include the last trench above per row, so need to add
        // one here.
        filled += count + 1;
    }

    filled as usize
}

fn to_verts(instructions: &[Instr]) -> Vec<(IPoint, isize)> {
    let mut current = pt(0, 0);
    let mut lines = vec![];

    for ins in instructions {
        match ins.dirn.y {
            -1 => lines.push((current + ins.dirn * ins.distance, ins.distance)),
            1 => lines.push((current, ins.distance)),
            0 => {}
            _ => panic!(),
        }
        current += ins.dirn * ins.distance;
    }

    lines.sort_by_key(|(p, _d)| p.x);
    lines
}

// Key for hashmap is the **Y** coordinate of the horizontals.
fn to_horz(instructions: &[Instr]) -> HashMap<isize, Vec<(IPoint, isize)>> {
    let mut current = pt(0, 0);
    let mut lines = HashMap::<isize, Vec<_>>::new();

    for ins in instructions {
        match ins.dirn.x {
            -1 => lines
                .entry(current.y)
                .or_default()
                .push((current + ins.dirn * ins.distance, ins.distance)),
            1 => lines
                .entry(current.y)
                .or_default()
                .push((current, ins.distance)),
            0 => {}
            _ => panic!(),
        }
        current += ins.dirn * ins.distance;
    }

    for v in lines.values_mut() {
        v.sort_by_key(|(p, _d)| p.x);
    }

    lines
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
    use aoc::lines_from_str;

    use crate::*;

    #[test]
    fn square() {
        let input = r#"R 4 (#123456)
D 3 (#123456)
L 4 (#123456)
U 3 (#123456)"#;
        let input = lines_from_str(input);
        let instructions = parse_instructions(input);
        let count = sparse_fill(&instructions);
        assert_eq!(20, count);
    }

    #[test]
    fn lshape() {
        let input = r#"R 4 (#123456)
D 3 (#123456)
R 4 (#123456)
D 3 (#123456)
L 8 (#123456)
U 6 (#123456)"#;
        let input = lines_from_str(input);
        let instructions = parse_instructions(input);

        let count = sparse_fill(&instructions);
        assert_eq!(36 + 15, count);
    }

    #[test]
    fn nshape() {
        let input = r#"R 12 (#123456)
D 6 (#123456)
L 3 (#123456)
U 3 (#123456)
L 6 (#123456)
D 3 (#123456)
L 3 (#123456)
U 6 (#123456)"#;
        let input = lines_from_str(input);
        let instructions = parse_instructions(input);

        let count = sparse_fill(&instructions);
        assert_eq!(12 + 12 + 4 * 13, count);
    }

    #[test]
    fn up_l() {
        let input = r#"R 4 (#123456)
U 4 (#123456)
R 4 (#123456)
D 8 (#123456)
L 8 (#123456)
U 4 (#123456)
"#;
        let input = lines_from_str(input);
        let instructions = parse_instructions(input);

        let count = sparse_fill(&instructions);
        assert_eq!(20 + 5 * 9, count);
    }

    #[test]
    fn example() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
        let input = lines_from_str(input);
        let instructions = parse_instructions(input);

        let count = sparse_fill(&instructions);
        assert_eq!(62, count);
    }

    #[test]
    fn saw() {
        let input = r#"D 6 (#70c710)
R 2 (#70c710)
U 3 (#70c710)
R 2 (#70c710)
D 3 (#70c710)
R 2 (#70c710)
U 3 (#70c710)
R 2 (#70c710)
D 3 (#70c710)
R 2 (#70c710)
U 3 (#70c710)
R 2 (#70c710)
D 3 (#70c710)
R 2 (#70c710)
U 6 (#70c710)
L 14 (#70c710)"#;

        let input = lines_from_str(input);
        let instructions = parse_instructions(input);

        let count = sparse_fill(&instructions);
        assert_eq!(15 * 7 - 9, count);
    }
}
