use std::collections::HashMap;

use aoc::{
    lines,
    two::{pt, Point},
};
use itertools::Itertools;

fn main() {
    let input = lines("input/day11");
    let galaxies = get_galaxies(input);
    let part1 = sum_distances(galaxies.clone(), 2);
    let part2 = sum_distances(galaxies.clone(), 1000000);
    dbg!(part1);
    dbg!(part2);
}

fn sum_distances(galaxies: Vec<Point<isize>>, expand_factor: isize) -> usize {
    let xs: Vec<_> = galaxies.iter().map(|p| p.x).collect();
    let ys: Vec<_> = galaxies.iter().map(|p| p.y).collect();
    let xguide = expand_guide(xs, expand_factor);
    let yguide = expand_guide(ys, expand_factor);

    // Crazy iterator type!
    let galaxies = galaxies.into_iter().map(|p| pt(xguide[&p.x], yguide[&p.y]));
    let pairs = galaxies.combinations(2);
    let distances = pairs.map(|p| p[0].taxicab_dist(p[1]));

    distances.sum()
}

/// Given the galaxies along one dimension, tells you how to map each coordinate
/// to the expanded coordinate.
fn expand_guide(mut xs: Vec<isize>, expand_factor: isize) -> HashMap<isize, isize> {
    xs.sort();
    let mut expand_guide = HashMap::<isize, isize>::new();
    expand_guide.insert(xs[0], 0);

    let mut last = xs[0];
    let mut total_expansion = 0;

    for &x in &xs[1..] {
        let d = x - last;
        if d == 0 {
            continue; // done this one.
        }

        total_expansion += (d - 1) * expand_factor + 1;
        expand_guide.insert(x, total_expansion);
        last = x;
    }

    expand_guide
}

fn get_galaxies(input: Vec<String>) -> Vec<Point<isize>> {
    let mut galaxies: Vec<Point<isize>> = vec![];
    for (y, line) in input.iter().enumerate() {
        for (x, &c) in line.as_bytes().iter().enumerate() {
            if c == b'#' {
                galaxies.push(pt(x as isize, y as isize));
            }
        }
    }
    galaxies
}

#[cfg(test)]
mod test {
    use aoc::lines_from_str;

    use crate::*;

    #[test]
    fn answers() {
        let input = lines("input/day11");
        let galaxies = get_galaxies(input);
        let part1 = sum_distances(galaxies.clone(), 2);
        let part2 = sum_distances(galaxies.clone(), 1000000);
        assert_eq!(9957702, part1);
        assert_eq!(512240933238, part2);
    }

    #[test]
    fn test_expand() {
        let guide = expand_guide(vec![3, 6, 8], 2);
        assert_eq!(0, guide[&3isize]);
        assert_eq!(5, guide[&6isize]);
        assert_eq!(8, guide[&8isize]);
    }

    #[test]
    fn test_dist() {
        let input = lines_from_str(r#"...#..#..."#);
        let galaxies = get_galaxies(input);
        let part1 = sum_distances(galaxies.clone(), 2);
        assert_eq!(part1, 5);
    }

    #[test]
    fn test_example() {
        let input = lines_from_str(
            r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#,
        );
        let galaxies = get_galaxies(input);
        let part1 = sum_distances(galaxies.clone(), 2);
        assert_eq!(part1, 374);
    }

    #[test]
    fn test_dist2() {
        let input = lines_from_str(r#"...#..#...#"#);
        let galaxies = get_galaxies(input);
        let part1 = sum_distances(galaxies.clone(), 2);
        assert_eq!(part1, 5 + 7 + 12);
    }

    #[test]
    fn test_dist3() {
        let input = lines_from_str(
            r#"
...#
....
...#
....
...#"#,
        );
        let galaxies = get_galaxies(input);
        let part1 = sum_distances(galaxies.clone(), 2);
        assert_eq!(part1, 3 + 3 + 6);
    }

    #[test]
    fn test_diag() {
        let input = lines_from_str(
            r#"
...#......
....#....."#,
        );
        let galaxies = get_galaxies(input);
        let part1 = sum_distances(galaxies.clone(), 2);
        assert_eq!(part1, 2);
    }

    #[test]
    fn test_diag2() {
        let input = lines_from_str(
            r#"
...#......
..........
....#....."#,
        );
        let galaxies = get_galaxies(input);
        let part1 = sum_distances(galaxies.clone(), 2);
        assert_eq!(part1, 4);
    }

    #[test]
    fn test_diag3() {
        let input = lines_from_str(
            r#"
...#......
..........
......#..."#,
        );
        let galaxies = get_galaxies(input);
        let part1 = sum_distances(galaxies.clone(), 2);
        assert_eq!(part1, 8);
    }
}
