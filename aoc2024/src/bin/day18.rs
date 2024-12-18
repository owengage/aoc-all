use core::panic;
use std::{
    collections::BinaryHeap,
    fmt::{Display, Write},
};

use aoc::{
    fetch_input, lines,
    two::{pt, DenseField, IPoint},
    StrExt,
};
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Cell {
    Empty { visited: bool, dist: usize },
    Corrupt,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty { visited, .. } => {
                if *visited {
                    f.write_char('o')
                } else {
                    f.write_char('.')
                }
            }
            Cell::Corrupt => f.write_char('#'),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Node {
    dist: usize,
    p: IPoint,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then(self.p.x.cmp(&other.p.x))
            .then(self.p.y.cmp(&other.p.y))
    }
}

fn main() {
    let drops = parse_input(lines(fetch_input(2024, 18)));
    let mut field = DenseField::new(
        71,
        71,
        Cell::Empty {
            visited: false,
            dist: usize::MAX,
        },
    );

    for drop in &drops[..1024] {
        *field.get_mut(*drop) = Cell::Corrupt;
    }

    let end = pt(70, 70);
    let mut blocking_drop = pt(0, 0);

    dijkstra(&mut field);
    println!(
        "part1 = {}",
        match field.get(end) {
            Cell::Empty { dist, .. } => *dist,
            _ => panic!(),
        }
    );

    for drop in &drops[1024..] {
        reset(&mut field);
        *field.get_mut(*drop) = Cell::Corrupt;

        dijkstra(&mut field);

        match field.get(end) {
            Cell::Empty { dist, .. } => {
                if *dist == usize::MAX {
                    blocking_drop = *drop;
                    break;
                }
            }
            Cell::Corrupt => todo!(),
        }
    }

    println!("part2 = {},{}", blocking_drop.x, blocking_drop.y);
}

fn reset(field: &mut DenseField<Cell>) {
    for y in 0..field.height() {
        for x in 0..field.width() {
            match field.get_mut(pt(x, y)) {
                Cell::Empty { visited, dist } => {
                    *visited = false;
                    *dist = usize::MAX;
                }
                Cell::Corrupt => {}
            }
        }
    }
}

fn dijkstra(field: &mut DenseField<Cell>) {
    let start = pt(0, 0);
    match field.get_mut(start) {
        Cell::Empty { dist, .. } => *dist = 0,
        Cell::Corrupt => panic!(),
    };

    let mut pq = BinaryHeap::<Node>::new();
    pq.push(Node { dist: 0, p: start });

    while let Some(current) = pq.pop() {
        let neighs = field
            .neighbours4_bounded(current.p)
            .map(|(nc, np)| (nc.clone(), np))
            .collect_vec();

        let current_dist = match field.get(current.p) {
            Cell::Empty { visited, dist } => {
                if *visited {
                    continue;
                } else {
                    *dist
                }
            }
            Cell::Corrupt => panic!(),
        };

        for (nc, np) in neighs {
            match nc {
                Cell::Empty { visited, dist } => {
                    if !visited && current_dist + 1 < dist {
                        *field.get_mut(np) = Cell::Empty {
                            visited,
                            dist: current_dist + 1,
                        };
                        pq.push(Node {
                            dist: current_dist + 1,
                            p: np,
                        });
                    }
                }
                Cell::Corrupt => {}
            }
        }

        *field.get_mut(current.p) = Cell::Empty {
            visited: true,
            dist: current_dist,
        };
    }
}

fn parse_input(input: Vec<String>) -> Vec<IPoint> {
    input
        .into_iter()
        .map(|l| {
            let s = l.as_str();
            let mut it = s.split_parse(",");
            pt(it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}
