use std::collections::HashSet;

use aoc::{fetch_input, text};

fn main() {
    // Written about this in obsidian notebook.
    let map = parse(&text(fetch_input(2022, 24)));

    let goalx = map.width - 1;
    let goaly = map.height - 1;

    // There, +1 because one more minute to actually get to the real target.
    let time = 1 + find_shortest(0, 0, 1, goalx, goaly, &map);
    // Back to get snacks. +1 to time because that will be the first minute we
    // can try to get there. Another +1 to go from 0,0 to actual start.
    let time = 1 + find_shortest(goalx, goaly, time + 1, 0, 0, &map);

    // // Back to the goal. +1 to go to inital 0,0, then +1 to get to real goal.
    let time = 1 + find_shortest(0, 0, time + 1, map.width - 1, map.height - 1, &map);

    // too high: 794

    // part2: too high: 2412
    println!("there: {}", time);
}

fn find_shortest(
    x: isize,
    y: isize,
    t: usize,
    target_x: isize,
    target_y: isize,
    map: &Map,
) -> usize {
    let mut start = Node { x, y, elapsed: t };

    println!("Searching start {:?}", start);

    while map.get_at(x, y, start.elapsed) != NONE {
        println!("Had to skip {:?}", start);
        start.elapsed += 1;
    }

    let first_elapsed_attempt = start.elapsed;

    let mut working = HashSet::new();
    working.insert(start);

    loop {
        let mut new_working = HashSet::new();

        if working.is_empty() {
            println!("Failed, elapsed: {}", first_elapsed_attempt);
            return find_shortest(x, y, first_elapsed_attempt + 1, target_x, target_y, map);
        }

        print_map(map, &working);

        for node in working {
            // println!("{:?}", node);

            if node.x == target_x && node.y == target_y {
                return node.elapsed;
            }

            expand(&mut new_working, &node, map);
        }

        working = new_working;
    }
}

fn print_map(map: &Map, working: &HashSet<Node>) {
    let t = working.iter().next().unwrap().elapsed;
    println!("elapsed: {}", t);

    for y in 0..map.height {
        for x in 0..map.width {
            let node = working.get(&Node { x, y, elapsed: t });

            let cell = map.get_at(x, y, t);
            if node.is_some() {
                assert_eq!(cell, NONE);
                print!("xx ");
            } else if cell == NONE {
                print!(".. ");
            } else {
                print!("{:02x} ", cell);
            }
        }

        println!();
        println!();
    }
}

fn expand(q: &mut HashSet<Node>, root: &Node, map: &Map) {
    let t = root.elapsed + 1;

    // right, down, nothing, up, left.

    // left
    if let Some(child) = make_child(root.x - 1, root.y, t, map) {
        q.insert(child);
    }
    // up
    if let Some(child) = make_child(root.x, root.y - 1, t, map) {
        q.insert(child);
    }
    // stay put
    if let Some(child) = make_child(root.x, root.y, t, map) {
        q.insert(child);
    }
    // down
    if let Some(child) = make_child(root.x, root.y + 1, t, map) {
        q.insert(child);
    }

    // right
    if let Some(child) = make_child(root.x + 1, root.y, t, map) {
        q.insert(child);
    }
}

fn make_child(x: isize, y: isize, t: usize, map: &Map) -> Option<Node> {
    if x >= 0 && x < map.width && y >= 0 && y < map.height && map.get_at(x, y, t) == NONE {
        Some(Node {
            x,
            y,
            elapsed: t,
            // children: vec![],
        })
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    x: isize,
    y: isize,
    elapsed: usize,
    // children: Vec<Node>,
}

fn parse(input: &str) -> Map {
    let height = (input.lines().count() - 2) as isize;
    let width = (input.lines().next().unwrap().len() - 2) as isize;

    let data = input
        .lines()
        .skip(1)
        .take(height as usize)
        .flat_map(|line| {
            line.chars()
                .skip(1)
                .take(width as usize)
                .map(|c| match c {
                    '>' => RIGHT,
                    '<' => LEFT,
                    '^' => UP,
                    'v' => DOWN,
                    '.' => NONE,
                    _ => panic!("{}", c),
                })
                .collect::<Vec<Cell>>()
        })
        .collect();

    Map {
        height,
        width,
        data,
    }
}

type Cell = u8;

#[derive(Debug, Clone)]
struct Map {
    data: Vec<Cell>,
    width: isize,
    height: isize,
}

impl Map {
    fn get_at(&self, x: isize, y: isize, t: usize) -> Cell {
        // Check directions offset by t from x,y for blizzards
        let mut cell = NONE;

        let leftx = (x + t as isize) % self.width;
        cell |= self.get_initial(leftx, y) & LEFT;

        let mut rightx = (x - t as isize) % self.width;
        if rightx < 0 {
            rightx += self.width;
        }
        cell |= self.get_initial(rightx, y) & RIGHT;

        let upy = (y + t as isize) % self.height;
        cell |= self.get_initial(x, upy) & UP;

        let mut downy = (y - t as isize) % self.height;
        if downy < 0 {
            downy += self.height;
        }
        cell |= self.get_initial(x, downy) & DOWN;

        cell
    }

    fn get_initial(&self, x: isize, y: isize) -> Cell {
        assert!(x >= 0 && x < self.width && y >= 0 && y < self.height);
        self.data[y as usize * self.width as usize + x as usize]
    }
}

const NONE: u8 = 0;
const LEFT: u8 = 1 << 0;
const RIGHT: u8 = 1 << 1;
const UP: u8 = 1 << 2;
const DOWN: u8 = 1 << 3;

// impl std::fmt::Debug for Cell {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("Cell(")?;

//         if self.0 & LEFT.0 != 0 {
//             f.write_char('<')?;
//         }
//         if self.0 & RIGHT.0 != 0 {
//             f.write_char('>')?;
//         }
//         if self.0 & UP.0 != 0 {
//             f.write_char('^')?;
//         }
//         if self.0 & DOWN.0 != 0 {
//             f.write_char('v')?;
//         }

//         if self.0 == 0 {
//             f.write_char('.')?;
//         }

//         f.write_char(')')
//     }
// }

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn get_bliz() {
        let map = parse(
            r#"############
#>......<..#
#.......v.v#
#...^..>.<.#
############"#,
        );

        assert_eq!(RIGHT, map.get_at(0, 0, 0));
        assert_eq!(RIGHT, map.get_at(0, 0, 123 * map.width as usize));
        assert_eq!(NONE, map.get_at(0, 0, 1));

        assert_eq!(LEFT, map.get_at(7, 0, 0));
        assert_eq!(LEFT, map.get_at(6, 0, 1));
        assert_eq!(RIGHT, map.get_at(6, 0, 6));

        assert_eq!(UP, map.get_at(3, 1, 1));
        assert_eq!(UP, map.get_at(3, 2, 0));
        assert_eq!(UP, map.get_at(3, 2, map.height as usize));
        assert_eq!(DOWN, map.get_at(9, 1, 0));
        assert_eq!(DOWN, map.get_at(9, 2, 1));

        assert_eq!(DOWN | LEFT | RIGHT, map.get_at(7, 2, 1));
    }

    #[test]
    fn negative_wrap() {
        let map = parse(
            r#"############
#.^........#
#..<.....>.#
#..........#
#..........#
#..........#
############"#,
        );

        assert_eq!(RIGHT, map.get_at(1, 1, 3));
        assert_eq!(RIGHT, map.get_at(1, 1, 3 + 2 * map.width as usize));

        assert_eq!(LEFT, map.get_at(1, 1, 1));
        assert_eq!(LEFT, map.get_at(1, 1, 1 + map.width as usize));

        assert_eq!(UP, map.get_at(1, 1, 4 + map.height as usize));
    }
}
