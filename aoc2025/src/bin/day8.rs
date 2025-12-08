use core::panic;
use std::collections::HashSet;

use aoc::{StrExt, fetch_input, lines};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    pub fn distance(self, other: Vec3) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        return ((dx * dx + dy * dy + dz * dz) as f64).sqrt();
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

    let part1 = part1(coords.clone(), 1000);
    println!("part1 = {part1}");
    assert_eq!(122636, part1);

    let part2 = part2(coords);
    println!("part2 = {part2}");
    assert_eq!(9271575747, part2);
}

fn part1(coords: Vec<Vec3>, connections_to_make: usize) -> usize {
    let pairs = make_pairs(&coords);
    let mut circuits = Vec::<HashSet<Vec3>>::new();

    for pair in pairs.iter().take(connections_to_make) {
        connect_pair(&mut circuits, *pair);
    }

    circuits.sort_by_key(|c| c.len());
    circuits.iter().rev().map(|c| c.len()).take(3).product()
}

fn part2(coords: Vec<Vec3>) -> usize {
    let pairs = make_pairs(&coords);
    let mut circuits = Vec::<HashSet<Vec3>>::new();

    for pair in pairs {
        connect_pair(&mut circuits, pair);

        // We've potentially just made a connection. Are we now fully connected?
        if circuits[0].len() == coords.len() {
            // Which junction boxes made this happen?
            let jb0 = pair.0;
            let jb1 = pair.1;
            return jb0.x as usize * jb1.x as usize;
        }
    }

    panic!("didn't fully connect");
}

// Make all pairs of coordinates, sorted from shortest distance to longest.
fn make_pairs(coords: &[Vec3]) -> Vec<(Vec3, Vec3)> {
    let pairs = coords
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|pair| {
            let d = pair.0.distance(*pair.1);
            (pair, d)
        })
        .sorted_by(|p1, p2| p1.1.partial_cmp(&p2.1).unwrap())
        .map(|p| (*p.0.0, *p.0.1))
        .collect_vec();
    pairs
}

// Given the existing circuits, connect the given pair if necessary.
fn connect_pair(circuits: &mut Vec<HashSet<Vec3>>, pair: (Vec3, Vec3)) {
    let p0in = circuits
        .iter()
        .find_position(|c| c.iter().contains(&pair.0));
    let p1in = circuits
        .iter()
        .find_position(|c| c.iter().contains(&pair.1));

    match (p0in, p1in) {
        (None, None) => {
            // neither are in a circuit, so make one.
            let c = HashSet::from_iter([pair.0, pair.1]);
            circuits.push(c);
        }
        (None, Some((i, _))) => {
            // one is in a circuit.
            circuits.get_mut(i).unwrap().insert(pair.0);
        }
        (Some((i, _)), None) => {
            // one is in a circuit.
            circuits.get_mut(i).unwrap().insert(pair.1);
        }
        (Some((i0, _)), Some((i1, _))) if i0 == i1 => {
            // in the same circuit already!
        }
        (Some((i0, _)), Some((i1, _))) => {
            // in different circuits
            let c1 = circuits.get(i1).unwrap().clone();
            let c0 = circuits.get_mut(i0).unwrap();
            for v in c1 {
                c0.insert(v);
            }
            circuits.remove(i1);
        }
    }
}

#[cfg(test)]
mod test {
    use aoc::StrExt;
    use itertools::Itertools;

    use crate::{Vec3, part1};

    #[test]
    fn example() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let coords = input
            .lines()
            .into_iter()
            .map(|s| {
                let mut it = s.split_parse::<isize>(",");
                Vec3 {
                    x: it.next().unwrap(),
                    y: it.next().unwrap(),
                    z: it.next().unwrap(),
                }
            })
            .collect_vec();

        println!("{:?}", coords);

        assert_eq!(40, part1(coords, 10));
    }
}
