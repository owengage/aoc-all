use aoc::{DisjointSet, StrExt, fetch_input, lines};
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

    let pairs = coords
        .iter()
        .enumerate()
        .tuple_combinations::<(_, _)>()
        .map(|pair| {
            let d = pair.0.1.distance_sq(*pair.1.1);
            (pair.0.0, pair.1.0, d)
        })
        .sorted_by(|p1, p2| p1.2.partial_cmp(&p2.2).unwrap())
        .map(|p| (p.0, p.1))
        .collect_vec();

    let part1 = part1(&coords, &pairs);
    println!("part1 = {part1}");
    assert_eq!(122636, part1);

    let part2 = part2(&coords, &pairs);
    println!("part2 = {part2}");
    assert_eq!(9271575747, part2);
}

fn part1(coords: &[Vec3], pairs: &[(usize, usize)]) -> usize {
    let mut ds = DisjointSet::with_singles(coords.len());

    for pair in pairs.iter().take(1000) {
        ds.merge(pair.0, pair.1);
    }

    let mut sizes = ds.all_lens();
    sizes.sort();
    sizes.iter().rev().take(3).product()
}

fn part2(coords: &[Vec3], pairs: &[(usize, usize)]) -> usize {
    let mut ds = DisjointSet::with_singles(coords.len());

    for pair in pairs {
        ds.merge(pair.0, pair.1);

        // Are we now fully connected?
        if ds.len_of(0) == coords.len() {
            return coords[pair.0].x as usize * coords[pair.1].x as usize;
        }
    }

    panic!("didn't fully connect");
}
