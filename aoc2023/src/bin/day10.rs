use core::panic;
use std::collections::{HashSet, VecDeque};

use aoc::{
    lines,
    two::{pt, DenseField, Point, DOWN, LEFT, RIGHT, UP},
};

#[derive(Debug, PartialEq, Clone)]
struct Cell {
    pipe: Pipe,
    shortest_dist: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pipe(u8); // down-up-left-right
impl Pipe {
    fn down(self) -> bool {
        self.0 & 0b1000 == 0b1000
    }
    fn up(self) -> bool {
        self.0 & 0b0100 == 0b0100
    }
    fn left(self) -> bool {
        self.0 & 0b0010 == 0b0010
    }
    fn right(self) -> bool {
        self.0 & 0b0001 == 0b0001
    }
}

fn main() {
    let input = lines("input/day10");
    let field = DenseField::<Cell>::from_lines(input);
    let start = field
        .find(&Cell {
            pipe: Pipe(0b1111),
            shortest_dist: usize::MAX,
        })
        .unwrap();

    let (visited, part1) = part1(field.clone(), start);
    dbg!(part1);

    let part2 = part2(start, field, visited);
    dbg!(part2); // 395 work too high
}

fn part2(
    start: Point<isize>,
    mut field: DenseField<Cell>,
    visited: HashSet<Point<isize>>,
) -> usize {
    // Part 2, just go across horizontally each row. If we have the 'down' pipe then flip
    // whether we're in or or out of the loop. For each line, start 'out' of the
    // loop, if we see a down pip toggle that. If the cell isn't part of the
    // loop and it's 'in' then we count it.
    let mut area = 0;

    // The start pipe is all-connected and therefore can have a down connection
    // that is not actually part of the loop. This breaks our counting, so we
    // convert the pipe into it's 'proper' connected form for the loop.
    fix_start_pipe(&mut field, start);

    for y in 0..field.height() {
        let mut in_loop = false;
        for x in 0..field.width() {
            let cell = field.get(pt(x, y));
            let cell_in_loop = visited.contains(&pt(x, y));
            if cell_in_loop && cell.pipe.down() {
                in_loop = !in_loop;
            }

            area += (!cell_in_loop && in_loop) as usize;
        }
    }

    area
}

fn fix_start_pipe(field: &mut DenseField<Cell>, start: Point<isize>) {
    // First we need to fix the start piece, as it screws up this way of
    // counting if we're in the loop or not.
    let mut new_start = Pipe(0);

    // down-up-left-right
    if let Some(next) = field.try_get(start + DOWN) {
        if next.pipe.up() {
            new_start.0 |= 0b1000;
        }
    }
    if let Some(next) = field.try_get(start + UP) {
        if next.pipe.down() {
            new_start.0 |= 0b0100;
        }
    }
    if let Some(next) = field.try_get(start + LEFT) {
        if next.pipe.right() {
            new_start.0 |= 0b0010;
        }
    }
    if let Some(next) = field.try_get(start + RIGHT) {
        if next.pipe.left() {
            new_start.0 |= 0b0001;
        }
    }

    assert_eq!(new_start.0.count_ones(), 2);
    *field.get_mut(start) = Cell {
        pipe: new_start,
        shortest_dist: 0,
    };
}

fn part1(mut field: DenseField<Cell>, start: Point<isize>) -> (HashSet<Point<isize>>, usize) {
    let mut q = VecDeque::new();
    let mut visited = HashSet::<Point<isize>>::new();

    q.push_back((start, 0));

    while let Some((pos, dist)) = q.pop_front() {
        visited.insert(pos);
        let cell = field.get(pos).clone();

        if dist < cell.shortest_dist {
            field.get_mut(pos).shortest_dist = dist;
        } else {
            continue; // Another path got here quicker, so don't bother exploring anymore.
        }

        if let Some(next) = field.try_get(pos + DOWN) {
            if cell.pipe.down() && next.pipe.up() {
                q.push_back((pt(pos.x, pos.y + 1), dist + 1));
            }
        }
        if let Some(next) = field.try_get(pos + UP) {
            if cell.pipe.up() && next.pipe.down() {
                q.push_back((pt(pos.x, pos.y - 1), dist + 1));
            }
        }
        if let Some(next) = field.try_get(pos + LEFT) {
            if cell.pipe.left() && next.pipe.right() {
                q.push_back((pt(pos.x - 1, pos.y), dist + 1));
            }
        }
        if let Some(next) = field.try_get(pos + RIGHT) {
            if cell.pipe.right() && next.pipe.left() {
                q.push_back((pt(pos.x + 1, pos.y), dist + 1));
            }
        }
    }

    let ans = visited
        .iter()
        .map(|p| field.get(*p).shortest_dist)
        .max()
        .unwrap();

    (visited, ans)
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        // down-up-left-right
        let pipe = match value {
            b'F' => Pipe(0b1001),
            b'L' => Pipe(0b0101),
            b'-' => Pipe(0b0011),
            b'|' => Pipe(0b1100),
            b'J' => Pipe(0b0110),
            b'7' => Pipe(0b1010),
            b'S' => Pipe(0b1111),
            b'.' => Pipe(0),
            _ => panic!("Found {}", value as char),
        };

        Cell {
            pipe,
            shortest_dist: usize::MAX,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::{lines_from_str, two::DenseField};

    #[test]
    fn test_parse() {
        let input = lines_from_str(
            r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#,
        );
        let field = DenseField::<Cell>::from_lines(input);
        let start = field
            .find(&Cell {
                pipe: Pipe(0b1111),
                shortest_dist: usize::MAX,
            })
            .unwrap();

        let (visited, _) = part1(field.clone(), start);

        let part2 = part2(start, field, visited);

        assert_eq!(10, part2);
    }
}
