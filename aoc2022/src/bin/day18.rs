use std::collections::{HashSet, VecDeque};

use aoc::{fetch_input, text};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Lava,
}

#[derive(Debug)]
struct Space {
    data: Vec<Cell>,
    height: isize,
    width: isize,
    depth: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Space {
    fn new(width: isize, height: isize, depth: isize) -> Self {
        Self {
            data: vec![Cell::Air; (height * width * depth) as usize],
            height,
            width,
            depth,
        }
    }

    fn get(&self, p: Point) -> Cell {
        let i = self.index(p);
        self.data[i]
    }

    fn get_inf(&self, p: Point) -> Cell {
        let i = self.index(p);
        if self.in_bounds(p) {
            self.data[i]
        } else {
            Cell::Air
        }
    }

    fn set(&mut self, p: Point, c: Cell) {
        let i = self.index(p);
        self.data[i] = c;
    }

    fn neighbours(&self, p: Point) -> [Cell; 6] {
        [
            self.get_inf(pn(p.x + 1, p.y, p.z)),
            self.get_inf(pn(p.x - 1, p.y, p.z)),
            self.get_inf(pn(p.x, p.y + 1, p.z)),
            self.get_inf(pn(p.x, p.y - 1, p.z)),
            self.get_inf(pn(p.x, p.y, p.z + 1)),
            self.get_inf(pn(p.x, p.y, p.z - 1)),
        ]
    }

    fn neighbour_points(&self, p: Point) -> [Option<Point>; 6] {
        let ps = [
            pn(p.x + 1, p.y, p.z),
            pn(p.x - 1, p.y, p.z),
            pn(p.x, p.y + 1, p.z),
            pn(p.x, p.y - 1, p.z),
            pn(p.x, p.y, p.z + 1),
            pn(p.x, p.y, p.z - 1),
        ];

        ps.map(|p| self.in_bounds(p).then_some(p))
    }

    fn index(&self, p: Point) -> usize {
        (p.z * (self.height * self.width) + p.y * self.width + p.x) as usize
    }

    fn in_bounds(&self, p: Point) -> bool {
        p.x >= 0
            && p.x < self.width
            && p.y >= 0
            && p.y < self.height
            && p.z >= 0
            && p.z < self.depth
    }

    fn flood(&self, start: Point) -> HashSet<Point> {
        assert_eq!(self.get(start), Cell::Air);
        let mut ps = HashSet::new();
        let mut q = VecDeque::new();

        q.push_back(start);

        while let Some(current) = q.pop_back() {
            ps.insert(current);
            let n = self.neighbour_points(current);

            for np in n.into_iter().flatten() {
                if self.get(np) == Cell::Air && !ps.contains(&np) {
                    q.push_back(np);
                }
            }
        }

        ps
    }
}

fn pn(x: isize, y: isize, z: isize) -> Point {
    Point { x, y, z }
}

fn main() {
    let input: Vec<_> = parse(&text(fetch_input(2022, 18)))
        .into_iter()
        .map(|p| Point {
            x: p.x + 1,
            y: p.y + 1,
            z: p.z + 1,
        })
        .collect();

    let bounds = input
        .iter()
        .copied()
        .reduce(|acc, p| Point {
            x: p.x.max(acc.x),
            y: p.y.max(acc.y),
            z: p.z.max(acc.z),
        })
        .unwrap();

    let mut space = Space::new(bounds.x + 5, bounds.y + 5, bounds.z + 5);

    for p in &input {
        space.set(*p, Cell::Lava);
    }

    // Okay, now we just want to count the amount of sides lava has air around
    // it?
    let mut exposed_total = 0;

    for p in &input {
        let exposed = space
            .neighbours(*p)
            .into_iter()
            .filter(|&c| c == Cell::Air)
            .count();

        exposed_total += exposed;
    }

    // correct: 4504
    println!("exposed: {:?}", exposed_total);

    let air = space.flood(pn(0, 0, 0));

    let mut exposed_total = 0;

    for p in &air {
        let exposed = space
            .neighbours(*p)
            .into_iter()
            .filter(|&c| c == Cell::Lava)
            .count();

        exposed_total += exposed;
    }

    println!("exposed: {:?}", exposed_total);
}

fn parse(data: &str) -> Vec<Point> {
    data.lines()
        .map(|p| {
            let ps: Vec<_> = p.split(',').flat_map(|ps| ps.parse::<isize>()).collect();
            Point {
                x: ps[0],
                y: ps[1],
                z: ps[2],
            }
        })
        .collect()
}
