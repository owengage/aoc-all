use core::panic;
use std::io::BufRead;

use aoc::{fetch_input, text};

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn below(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left_below(&self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn right_below(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

#[derive(Debug)]
struct Line(Vec<Coord>);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Map {
    data: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Map {
            data: vec![Cell::Air; width * height],
            width,
            height,
        }
    }

    fn set(&mut self, c: Coord, cell: Cell) {
        assert!((0..self.width as isize).contains(&c.x));
        assert!((0..self.height as isize).contains(&c.y));
        self.data[c.y as usize * self.width + c.x as usize] = cell;
    }

    fn get(&self, c: Coord) -> Option<Cell> {
        if c.y as usize == self.height - 1 {
            // Rock floor
            return Some(Cell::Rock);
        }

        if (0..self.width as isize).contains(&c.x) && (0..self.height as isize).contains(&c.y) {
            Some(self.data[c.y as usize * self.width + c.x as usize])
        } else {
            None
        }
    }
}

fn parse(r: impl BufRead) -> Vec<Line> {
    r.lines()
        .flatten()
        .map(|line| {
            line.split(" -> ")
                .map(|p| {
                    let mut it = p.split(',').map(|v| v.parse::<isize>().unwrap());
                    Coord {
                        x: it.next().unwrap(),
                        y: it.next().unwrap(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .map(Line)
        .collect()
}

fn main() {
    let lines = parse(text(fetch_input(2022, 14)).as_bytes());

    let bounds = lines.iter().flat_map(|line| line.0.clone()).fold(
        (
            Coord {
                x: isize::MAX,
                y: isize::MAX,
            },
            Coord {
                x: isize::MIN,
                y: isize::MIN,
            },
        ),
        |acc, p| {
            (
                Coord {
                    x: acc.0.x.min(p.x),
                    y: acc.0.y.min(p.y),
                },
                Coord {
                    x: acc.1.x.max(p.x),
                    y: acc.1.y.max(p.y),
                },
            )
        },
    );

    let xoffset = 0;

    // let xoffset = bounds.0.x;

    let lines = lines.into_iter().map(|line| {
        Line(
            line.0
                .into_iter()
                .map(|p| Coord {
                    x: p.x - xoffset,
                    y: p.y,
                })
                .collect(),
        )
    });

    let sand_origin = Coord {
        x: 500 - xoffset,
        y: 0,
    };

    let mut map = Map::new(
        (bounds.1.x - xoffset + 1 + bounds.1.y) as usize, // heap could be beyond.
        (bounds.1.y + 1 + 2) as usize, // +1 because 1-based, +2 because part 2 floor.
    );

    for line in lines {
        for seg in line.0.windows(2) {
            let seg = [seg[0], seg[1]];
            draw_line_segment(&mut map, seg)
        }
    }

    print_map(&map);

    // Simulate the sand!
    let mut count = 0;
    while simulate_sand_grain(&mut map, sand_origin) {
        count += 1;
        if count % 10000 == 0 {
            // print_map(&map);
        }
    }
    print_map(&map);

    println!("Sand grains: {count}");
    // println!("{:?}", lines);
}

fn simulate_sand_grain(map: &mut Map, sand_origin: Coord) -> bool {
    let mut current = sand_origin;

    if map.get(current).unwrap() == Cell::Sand {
        return false;
    }

    map.set(current, Cell::Sand);

    'outer: loop {
        for next in [current.below(), current.left_below(), current.right_below()] {
            match map.get(next) {
                Some(Cell::Air) => {
                    map.set(current, Cell::Air);
                    map.set(next, Cell::Sand);
                    current = next;
                    continue 'outer;
                }
                Some(Cell::Rock | Cell::Sand) => {}
                None => return false, // we fell off.
            }
        }

        // no spaces are free, so we're done?
        return true;
    }
}

fn print_map(map: &Map) {
    for y in 0..map.height {
        for x in 0..map.width {
            print!(
                "{}",
                match map
                    .get(Coord {
                        x: x as isize,
                        y: y as isize
                    })
                    .unwrap()
                {
                    Cell::Air => ' ',
                    Cell::Rock => '█',
                    Cell::Sand => 'x',
                }
            );
        }
        println!();
    }
}

fn draw_line_segment(map: &mut Map, seg: [Coord; 2]) {
    let [a, b] = seg;

    if a.x == b.x {
        let x = a.x;
        let mut r = [a.y, b.y];
        r.sort();
        let r = r[0]..=r[1];

        for y in r {
            map.set(Coord { x, y }, Cell::Rock);
        }
    } else if a.y == b.y {
        let y = a.y;
        let mut r = [a.x, b.x];
        r.sort();
        let r = r[0]..=r[1];

        for x in r {
            map.set(Coord { x, y }, Cell::Rock);
        }
    } else {
        panic!();
    }
}
