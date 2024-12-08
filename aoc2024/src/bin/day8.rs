use std::collections::HashMap;

use aoc::{
    fetch_input, lines,
    two::{DenseField, IPoint, Point},
};
use itertools::Itertools;

#[derive(Clone)]
struct Cell {
    antenna: Option<u8>,
    has_antinode: bool,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        Cell {
            antenna: value.is_ascii_alphanumeric().then_some(value),
            has_antinode: false,
        }
    }
}

fn main() {
    let mut field = DenseField::<Cell>::from_lines(lines(fetch_input(2024, 8)));
    let antennas = find_antennas(&field);

    // Don't need to clone the field, any antinodes in p1 are also in p2.
    let part1 = part1(&antennas, &mut field);
    let part2 = part2(&antennas, &mut field);

    println!("part1 = {part1}");
    println!("part2 = {part2}");
    assert_eq!(part1, 332);
    assert_eq!(part2, 1174);
}

fn part1(antennas: &HashMap<u8, Vec<IPoint>>, field: &mut DenseField<Cell>) -> usize {
    // For every pair of antenna, we need to calcualte where their antinodes
    // will be.
    for ps in antennas.values() {
        // Using permutations means we'll always see each pair twice, one for
        // each order.
        for pair in ps.iter().permutations(2) {
            let p1 = *pair[0];
            let p2 = *pair[1];
            let delta = p2 - p1;
            let antinode = p2 + delta;
            if let Some(cell) = field.try_get_mut(antinode.x, antinode.y) {
                cell.has_antinode = true;
            }
        }
    }

    field.data().iter().map(|c| c.has_antinode as usize).sum()
}

fn part2(antennas: &HashMap<u8, Vec<IPoint>>, field: &mut DenseField<Cell>) -> usize {
    for ps in antennas.values() {
        for pair in ps.iter().permutations(2) {
            let p1 = *pair[0];
            let p2 = *pair[1];
            let delta = p2 - p1;
            let mut antinode = p2; // starts at self now for part2.

            while let Some(cell) = field.try_get_mut(antinode.x, antinode.y) {
                cell.has_antinode = true;
                antinode += delta; // move on.
            }
        }
    }

    field.data().iter().map(|c| c.has_antinode as usize).sum()
}

fn find_antennas(field: &DenseField<Cell>) -> HashMap<u8, Vec<Point<isize>>> {
    let mut antenna = HashMap::<u8, Vec<IPoint>>::new();

    // Need to know where all the various frequency antenna are.
    for p in field.points() {
        let cell = field.get(p.x, p.y);
        if let Some(ant) = cell.antenna {
            antenna.entry(ant).or_default().push(p);
        }
    }
    antenna
}
