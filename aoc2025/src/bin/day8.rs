use aoc::{DisjointSet, StrExt, fetch_input, lines};
use itertools::Itertools;

struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    pub fn distance_sq(&self, other: &Vec3) -> isize {
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
            let [x, y, z] = s.as_str().split_parse_n::<3, isize>(",");
            Vec3 { x, y, z }
        })
        .collect_vec();

    let pairs = coords
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((i1, c1), (i2, c2))| ((i1, i2), c1.distance_sq(c2)))
        .sorted_by(|(_, d1), (_, d2)| d1.partial_cmp(&d2).unwrap())
        .map(|(ii, _)| ii)
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

    for &(a, b) in pairs.iter().take(1000) {
        ds.merge(a, b);
    }

    let mut lens = ds.all_lens();
    lens.sort();
    lens.iter().rev().take(3).product()
}

fn part2(coords: &[Vec3], pairs: &[(usize, usize)]) -> usize {
    let mut ds = DisjointSet::with_singles(coords.len());

    for &(a, b) in pairs {
        ds.merge(a, b);

        // Are we now fully connected?
        if ds.len_of(0) == coords.len() {
            return coords[a].x as usize * coords[b].x as usize;
        }
    }

    panic!("didn't fully connect");
}
