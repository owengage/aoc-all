use std::collections::VecDeque;

use aoc::{
    fetch_input, lines,
    two::{pt, DenseField, DOWN, LEFT, RIGHT, UP},
};
use itertools::Itertools;

#[derive(Clone, Copy)]
struct Cell {
    value: char,
    visited: bool,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        Cell {
            value: value as char,
            visited: false,
        }
    }
}

fn main() {
    let field = DenseField::<Cell>::from_lines(lines(fetch_input(2024, 12)));
    println!("part1 = {}", part1(field.clone()));
    println!("part2 = {}", part2(field));
}

fn part2(mut field: DenseField<Cell>) -> usize {
    let mut regions = Vec::<(usize, usize)>::new(); // area, verticies.
    let mut leads = VecDeque::new();
    leads.push_back(pt(0, 0));

    // Go until we've found all regions.
    while let Some(lead) = leads.pop_front() {
        let cell = *field.get(lead.x, lead.y);
        if cell.visited {
            continue; // already seen here.
        }

        let flood_value = cell.value;
        let mut area = 0;
        let mut verts = 0;
        let mut q = VecDeque::new();
        q.push_back(lead);

        // Flood fill this region.
        while let Some(p) = q.pop_front() {
            if field.get(p.x, p.y).visited {
                continue;
            }

            field.get_mut(p.x, p.y).visited = true;

            // We only got here if the value was equal to the flood we're
            // currently doing so we can add to the area.
            area += 1;

            // For each 2x2 quarter of the 3x3...
            for dirns in [UP, RIGHT, DOWN, LEFT, UP].windows(2) {
                let n1 = p + dirns[0]; // neighbour 'up'
                let n2 = p + dirns[1]; // neighbour 'right'
                let corner = p + dirns[0] + dirns[1]; // 'corner'

                let v1 = field.try_get(n1.x, n1.y).map(|c| c.value).unwrap_or('.');
                let v2 = field.try_get(n2.x, n2.y).map(|c| c.value).unwrap_or('.');
                let vc = field
                    .try_get(corner.x, corner.y)
                    .map(|c| c.value)
                    .unwrap_or('.');

                // Vertex if both up/right are not equal to us.
                let exterior_corner = v1 != cell.value && v2 != cell.value;

                // Vertex if both up/right ARE equal to us, but the corner is not.
                let interior_corner = v1 == cell.value && v2 == cell.value && vc != cell.value;

                if exterior_corner || interior_corner {
                    verts += 1;
                }
            }

            // Actually flood fill still.
            let neigh = field.neighbours4_bounded(p.x, p.y).collect_vec();
            for (ncell, n) in neigh {
                if ncell.value == flood_value {
                    q.push_back(n);
                } else {
                    leads.push_back(n);
                }
            }
        }

        regions.push((area, verts));
    }

    regions
        .iter()
        .map(|(area, verts)| area * verts)
        .sum::<usize>()
}

fn part1(mut field: DenseField<Cell>) -> usize {
    let mut regions = Vec::<(usize, usize)>::new();
    let mut leads = VecDeque::new();
    // leads to look for new regions.
    leads.push_back(pt(0, 0));

    // Flood fill each region and add to leads as we go. A non-same neighbour
    // adds 1 to the perim INCLUDING field boundaries, and valeus of same add
    // one to area.
    while let Some(lead) = leads.pop_front() {
        let cell = field.get(lead.x, lead.y);
        if cell.visited {
            continue; // already seen here.
        }

        let flood_value = cell.value;
        let mut area = 0;
        let mut perim = 0;
        let mut q = VecDeque::new();
        q.push_back(lead);

        while let Some(p) = q.pop_front() {
            // println!("Visiting {:?}", p);
            if field.get(p.x, p.y).visited {
                continue;
            }

            field.get_mut(p.x, p.y).visited = true;

            let neigh = field.neighbours4_bounded(p.x, p.y).collect_vec();
            perim += 4 - neigh.len(); // account for any borders.
            area += 1;

            for (ncell, n) in neigh {
                if ncell.value == flood_value {
                    q.push_back(n);
                } else {
                    // we're bordering with another region, count our fence.
                    perim += 1;
                    leads.push_back(n);
                }
            }
        }

        regions.push((area, perim));
    }

    regions
        .iter()
        .map(|(area, perim)| area * perim)
        .sum::<usize>()
}
