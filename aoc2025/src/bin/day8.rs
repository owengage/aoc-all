use core::panic;
use std::{collections::HashSet, mem};

use aoc::{StrExt, fetch_input, lines};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    pub fn distance_sq(self, other: Vec3) -> isize {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        return dx * dx + dy * dy + dz * dz;
    }
}

fn main() {
    let input = lines(fetch_input(2025, 8));
    let coords = input
        .into_iter()
        .map(|s| {
            let s = s.as_str();
            let mut it = s.split_parse::<isize>(",");
            Vec3 {
                x: it.next().unwrap(),
                y: it.next().unwrap(),
                z: it.next().unwrap(),
            }
        })
        .collect_vec();

    let pairs = make_pairs(&coords);
    let circuits = init_circuits(&coords);

    let part1 = part1(&pairs, circuits.clone());
    println!("part1 = {part1}");
    assert_eq!(122636, part1);

    let part2 = part2(&pairs, circuits);
    println!("part2 = {part2}");
    assert_eq!(9271575747, part2);
}

fn part1(pairs: &Vec<(Vec3, Vec3)>, mut circuits: Vec<HashSet<Vec3>>) -> usize {
    for pair in pairs.iter().take(1000) {
        connect_pair(&mut circuits, *pair);
    }

    circuits.sort_by_key(|c| c.len());
    circuits.iter().rev().map(|c| c.len()).take(3).product()
}

fn part2(pairs: &Vec<(Vec3, Vec3)>, mut circuits: Vec<HashSet<Vec3>>) -> usize {
    let coord_count = circuits.len();
    for pair in pairs {
        connect_pair(&mut circuits, *pair);

        // Are we now fully connected?
        if circuits[0].len() == coord_count {
            return pair.0.x as usize * pair.1.x as usize;
        }
    }

    panic!("didn't fully connect");
}

/// Make a circuit for each coordinate.
fn init_circuits(coords: &Vec<Vec3>) -> Vec<HashSet<Vec3>> {
    Vec::<HashSet<Vec3>>::from_iter(coords.iter().map(|c| HashSet::from_iter([*c])))
}

/// Make all pairs of coordinates, sorted from shortest distance to longest.
fn make_pairs(coords: &[Vec3]) -> Vec<(Vec3, Vec3)> {
    let pairs = coords
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|pair| {
            let d = pair.0.distance_sq(*pair.1);
            (pair, d)
        })
        .sorted_by(|p1, p2| p1.1.partial_cmp(&p2.1).unwrap())
        .map(|p| (*p.0.0, *p.0.1))
        .collect_vec();
    pairs
}

// Given the existing circuits, connect the given pair if necessary.
fn connect_pair(circuits: &mut Vec<HashSet<Vec3>>, pair: (Vec3, Vec3)) {
    let i0 = find_circuit(circuits, pair.0);
    let i1 = find_circuit(circuits, pair.1);

    // join different circuits
    if i0 != i1 {
        let stolen = mem::take(circuits.get_mut(i1).unwrap());
        circuits.get_mut(i0).unwrap().extend(stolen);
        circuits.remove(i1);
    }
}

// Find index of circuit that coord is in.
fn find_circuit(circuits: &mut Vec<HashSet<Vec3>>, coord: Vec3) -> usize {
    circuits
        .iter()
        .find_position(|c| c.contains(&coord))
        .unwrap()
        .0
}
