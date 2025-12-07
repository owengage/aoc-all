use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Sub},
};

fn parse(r: impl BufRead) -> Vec<(char, i32)> {
    r.lines()
        .flatten()
        .map(|line| {
            let (c, d) = line.split_once(' ').unwrap();
            (c.chars().next().unwrap(), d.parse::<i32>().unwrap())
        })
        .collect()
}

fn dir_to_coords(dir: char) -> Cell {
    match dir {
        'U' => Cell { x: 0, y: 1 },
        'D' => Cell { x: 0, y: -1 },
        'L' => Cell { x: -1, y: 0 },
        'R' => Cell { x: 1, y: 0 },
        _ => panic!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    fn origin() -> Cell {
        Cell { x: 0, y: 0 }
    }

    fn signum(&self) -> Cell {
        Cell {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl Add<Cell> for Cell {
    type Output = Cell;

    fn add(mut self, rhs: Cell) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl AddAssign<Cell> for Cell {
    fn add_assign(&mut self, rhs: Cell) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Cell> for Cell {
    type Output = Cell;

    fn sub(mut self, rhs: Cell) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

// diff = from - to

fn sim_rope(rope: &mut [Cell], moves: &[(char, i32)]) -> usize {
    let mut tail_set = HashSet::new();
    tail_set.insert(*rope.last().unwrap());

    for (dir, steps) in moves {
        for _ in 0..*steps {
            rope[0] += dir_to_coords(*dir);

            // propagate movement down rest of rope.
            for i in 0..rope.len() - 1 {
                let diff = rope[i] - rope[i + 1];

                if diff.x.abs() == 2 || diff.y.abs() == 2 {
                    rope[i + 1] += diff.signum();
                }
            }

            tail_set.insert(*rope.last().unwrap());
        }
    }

    tail_set.len()
}

fn main() {
    let moves = parse(BufReader::new(File::open("input.txt").unwrap()));
    println!("part 1: {}", sim_rope(&mut [Cell::origin(); 2], &moves));
    println!("part 2: {}", sim_rope(&mut [Cell::origin(); 10], &moves));
}
