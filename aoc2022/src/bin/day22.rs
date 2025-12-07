use std::collections::HashMap;

use aoc::{fetch_input, text};

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<Cell>,
    warps: HashMap<Warp, Warp>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Warp {
    side: usize,
    edge: Facing,
}
impl Warp {
    fn new(side: usize, edge: Facing) -> Warp {
        Warp { side, edge }
    }
}

fn warps() -> HashMap<Warp, Warp> {
    use Facing::*;
    let mut map = HashMap::new();
    map.insert(Warp::new(1, Up), Warp::new(9, Left)); // x -> side - y
    map.insert(Warp::new(1, Right), Warp::new(2, Left)); // y -> y
    map.insert(Warp::new(1, Down), Warp::new(4, Up));
    map.insert(Warp::new(1, Left), Warp::new(6, Left));
    map.insert(Warp::new(2, Up), Warp::new(9, Down));
    map.insert(Warp::new(2, Right), Warp::new(7, Right));
    map.insert(Warp::new(2, Down), Warp::new(4, Right));
    map.insert(Warp::new(4, Down), Warp::new(7, Up));
    map.insert(Warp::new(4, Left), Warp::new(6, Up));
    map.insert(Warp::new(6, Right), Warp::new(7, Left)); // y -> y
    map.insert(Warp::new(6, Down), Warp::new(9, Up));
    map.insert(Warp::new(7, Down), Warp::new(9, Right));

    map.into_iter()
        .flat_map(|(k, v)| [(k, v), (v, k)])
        .collect()
}

impl Map {
    fn new(width: usize, height: usize, data: Vec<Cell>) -> Map {
        Self {
            width,
            height,
            data,
            warps: warps(),
        }
    }

    fn get(&self, p: (isize, isize)) -> Cell {
        if p.0 < 0 || (p.0 as usize) >= self.width || p.1 < 0 || p.1 as usize >= self.height {
            return Cell::Warp;
        }
        self.data[p.1 as usize * self.width + p.0 as usize]
    }

    // Gets a position, warping and returning the real position.
    fn get_warped(&self, p: (isize, isize), facing: Facing) -> ((isize, isize), Facing, Cell) {
        let cell = self.get(p);

        match cell {
            Cell::Free(_) => (p, facing, cell),
            Cell::Wall => (p, facing, cell),
            Cell::Warp => {
                //
                // Part 2:
                //

                // Need to get our current cube side
                //
                // This point is already outside the bounds of the side.
                // We need to step back into the side, can go opposite the
                // facing direction.
                let (prev_x, prev_y) = peek(p, facing.invert());
                let cube_x = prev_x / CUBE_SIDE;
                let cube_y = prev_y / CUBE_SIDE;
                let side = (cube_y * 3 + cube_x) as usize;

                // Get where we're warping to
                let warp_from = Warp { side, edge: facing };
                let warp_to = self.warps[&warp_from];

                // What are our side-local coordinates?
                let local_x = prev_x % CUBE_SIDE;
                let local_y = prev_y % CUBE_SIDE;
                let w = CUBE_SIDE; // cube side length;
                let w1 = w - 1;

                // What do these coordinates transform into?
                let warped = match facing {
                    Facing::Up => match warp_to.edge {
                        Facing::Up => (w1 - local_x, 0),
                        Facing::Down => (local_x, w1),
                        Facing::Left => (0, local_x),
                        Facing::Right => (w1, w1 - local_x),
                    },
                    Facing::Down => match warp_to.edge {
                        Facing::Up => (local_x, 0),
                        Facing::Down => (w1 - local_x, w1),
                        Facing::Left => (0, w1 - local_x),
                        Facing::Right => (w1, local_x),
                    },
                    Facing::Left => match warp_to.edge {
                        Facing::Up => (local_y, 0),
                        Facing::Down => (w1, w1 - local_y),
                        Facing::Left => (0, w1 - local_y),
                        Facing::Right => (w1, local_y),
                    },
                    Facing::Right => match warp_to.edge {
                        Facing::Up => (w1 - local_y, 0),
                        Facing::Down => (local_y, w1),
                        Facing::Left => (0, local_y),
                        Facing::Right => (w1, w1 - local_y),
                    },
                };

                // Note: The warp_to.edge is the inverse of the new direction we
                // should be facing.

                // warped is local, need to get the delta to add from the side
                // number.
                let side = warp_to.side;
                let x = CUBE_SIDE * (side % 3) as isize + warped.0;
                let y = CUBE_SIDE * (side / 3) as isize + warped.1;
                let warped = (x, y);

                (warped, warp_to.edge.invert(), self.get(warped))

                // Part 1:
                // We need to find our real destination. We need to find the
                // free/wall cell before the next warp block we see, travelling
                // in the opposite direction to the way we're facing.
                // let search_dir = facing.invert();
                // let mut next = peek(p, search_dir);
                // while self.get(next) != Cell::Warp {
                //     next = peek(next, search_dir);
                // }
                // let previous = peek(next, facing);
                // (previous, self.get(previous))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Free(char),
    Wall,
    Warp,
}

#[derive(Debug)]
enum Instr {
    Left,
    Right,
    Walk(usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn turn_left(&mut self) {
        *self = match *self {
            Facing::Up => Facing::Left,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Right => Facing::Up,
        };
    }

    fn turn_right(&mut self) {
        *self = match *self {
            Facing::Up => Facing::Right,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Right => Facing::Down,
        };
    }

    fn invert(&self) -> Facing {
        match self {
            Facing::Up => Facing::Down,
            Facing::Down => Facing::Up,
            Facing::Left => Facing::Right,
            Facing::Right => Facing::Left,
        }
    }
}

const CUBE_SIDE: isize = 50;

fn main() {
    let (mut map, instr) = parse(&text(fetch_input(2022, 22)));
    let part1 = part2(&mut map, &instr);

    // print_map(&map);

    // too low: 1264
    println!("part 2: {}", part1);
}

fn part2(map: &mut Map, instructions: &[Instr]) -> isize {
    // find starting position
    let mut pos = get_start_position(map);
    let mut facing = Facing::Right;

    for ins in instructions {
        match ins {
            Instr::Left => facing.turn_left(),
            Instr::Right => facing.turn_right(),
            Instr::Walk(n) => {
                for _ in 0..*n {
                    let next = peek(pos, facing);
                    let (next, next_facing, cell) = map.get_warped(next, facing);
                    match cell {
                        Cell::Free(_) => {
                            // map.set(pos, Cell::Free(facing.symbol()));

                            facing = next_facing;
                            pos = next;
                        }
                        Cell::Wall => break,
                        Cell::Warp => panic!(),
                    }
                }
            }
        }
    }

    let row = pos.1 + 1;
    let col = pos.0 + 1;
    let facing = match facing {
        Facing::Up => 3,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Right => 0,
    };

    row * 1000 + 4 * col + facing
}

fn peek(pos: (isize, isize), facing: Facing) -> (isize, isize) {
    match facing {
        Facing::Up => (pos.0, pos.1 - 1),
        Facing::Down => (pos.0, pos.1 + 1),
        Facing::Left => (pos.0 - 1, pos.1),
        Facing::Right => (pos.0 + 1, pos.1),
    }
}

fn get_start_position(map: &Map) -> (isize, isize) {
    for x in 0..map.width {
        if matches!(map.get((x as isize, 0)), Cell::Free(_)) {
            return (x as isize, 0);
        }
    }

    panic!()
}

fn parse(input: &str) -> (Map, Vec<Instr>) {
    let mut lines: Vec<_> = input.lines().collect();
    let raw_instructions = lines.pop().unwrap();
    while lines.last().unwrap().is_empty() {
        lines.pop();
    }

    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap();

    let data: Vec<Cell> = lines
        .into_iter()
        .flat_map(|line| {
            let mut line = line.as_bytes().to_vec();
            line.resize(width, b' ');
            line
        })
        .map(|b| match b {
            b' ' => Cell::Warp,
            b'#' => Cell::Wall,
            b'.' => Cell::Free('.'),
            _ => panic!(),
        })
        .collect();

    let mut instrs = vec![];
    let mut token = String::new();
    for c in raw_instructions.chars() {
        if c.is_ascii_digit() {
            token.push(c);
            continue;
        }

        if !token.is_empty() {
            instrs.push(Instr::Walk(token.parse().unwrap()));
            token.clear();
        }

        if c == 'R' {
            instrs.push(Instr::Right);
        }

        if c == 'L' {
            instrs.push(Instr::Left);
        }
    }

    if !token.is_empty() {
        instrs.push(Instr::Walk(token.parse().unwrap()));
    }

    (Map::new(width, height, data), instrs)
}
