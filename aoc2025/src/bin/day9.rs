use std::ops::Range;

use aoc::{
    StrExt, fetch_input, lines,
    two::{IPoint, pt},
};
use itertools::Itertools;

struct Context {
    tiles: Vec<IPoint>,
    xs: Vec<(isize, usize)>, // (x value, tile index), sorted by x.
    ys: Vec<(isize, usize)>, // (y value, tile index), sorted by y.
}

impl Context {
    fn new(tiles: Vec<IPoint>) -> Self {
        let mut xs = tiles
            .iter()
            .enumerate()
            .map(|(i, t)| (t.x, i))
            .collect_vec();
        xs.sort_by_key(|t| t.0); // sort by x value.

        let mut ys = tiles
            .iter()
            .enumerate()
            .map(|(i, t)| (t.y, i))
            .collect_vec();
        ys.sort_by_key(|t| t.0); // sort by y value.

        Context { tiles, xs, ys }
    }
}

fn main() {
    let input = lines(fetch_input(2025, 9));
    let tiles = input
        .iter()
        .map(|t| {
            let [x, y]: [isize; 2] = t.as_str().split_parse_n(",");
            pt(x, y)
        })
        .collect_vec();

    let part1 = part1(&tiles);
    dbg!(part1);

    // Kinda of incorrect but works for the input we're given. Technically a
    // concave shape could be seen as 'inside' and be the max area, eg some sort
    // of horseshoe shape.
    let part2 = part2(&tiles);
    dbg!(part2);
}

fn part2(tiles: &[IPoint]) -> isize {
    let ctx = Context::new(tiles.to_vec());
    let mut current_max = 0;

    for (&a, &b) in tiles.iter().tuple_combinations() {
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();
        let candidate_area = (dx + 1) * (dy + 1);

        if candidate_area <= current_max {
            continue;
        }

        if is_candidate_valid(&ctx, a, b) {
            current_max = candidate_area;
        }
    }

    current_max
}

fn is_candidate_valid(ctx: &Context, a: IPoint, b: IPoint) -> bool {
    let xmin = a.x.min(b.x);
    let xmax = a.x.max(b.x);
    let ymin = a.y.min(b.y);
    let ymax = a.y.max(b.y);

    // TODO: Inside outside.
    // Are we inside?
    // How many lines do we cross if we draw a line from the edge to the
    // candidate midpoint?

    // For each point, does the point before or after it in the tile list cross
    // into our candidate box in the y direction?
    let xs_range = within_range(&ctx.xs, xmin, xmax);
    for &(x, ti) in &ctx.xs[xs_range] {
        let current = ctx.tiles[ti];
        let next = ctx.tiles[(ti + 1).rem_euclid(ctx.tiles.len())];
        let prev = ctx.tiles[(ti as isize - 1).rem_euclid(ctx.tiles.len() as isize) as usize];

        if next.x == x {
            // This is vertical line. Does it cross our candidate at all? Both y
            // values must be <= ymin OR both >= ymax. Equal is fine.
            let ok = (next.y <= ymin && current.y <= ymin) || (next.y >= ymax && current.y >= ymax);
            if !ok {
                return false;
            }
        }

        if prev.x == x {
            let ok = (prev.y <= ymin && current.y <= ymin) || (prev.y >= ymax && current.y >= ymax);
            if !ok {
                return false;
            }
        }
    }

    let ys_range = within_range(&ctx.ys, ymin, ymax);
    for &(y, ti) in &ctx.ys[ys_range] {
        let current = ctx.tiles[ti];
        let next = ctx.tiles[(ti + 1).rem_euclid(ctx.tiles.len())];
        let prev = ctx.tiles[(ti as isize - 1).rem_euclid(ctx.tiles.len() as isize) as usize];

        if next.y == y {
            // This is horizontal line. Does it cross our candidate at all? Both x
            // values must be <= ymin OR both >= ymax. Equal is fine.
            let ok = (next.x <= xmin && current.x <= xmin) || (next.x >= xmax && current.x >= xmax);
            if !ok {
                return false;
            }
        }

        if prev.y == y {
            let ok = (prev.x <= xmin && current.x <= xmin) || (prev.x >= xmax && current.x >= xmax);
            if !ok {
                return false;
            }
        }
    }

    true
}

fn within_range(vals: &[(isize, usize)], xmin: isize, xmax: isize) -> Range<usize> {
    let mut possible_min_i = vals.binary_search_by_key(&xmin, |v| v.0).unwrap();
    loop {
        possible_min_i += 1;
        if possible_min_i == vals.len() {
            break;
        }

        let next = vals[possible_min_i];
        if next.0 > xmin {
            break;
        }
    }

    let mut possible_max_i = vals.binary_search_by_key(&xmax, |v| v.0).unwrap();
    loop {
        if possible_max_i == 0 {
            break;
        }
        possible_max_i -= 1;

        let prev = vals[possible_max_i];
        if prev.0 < xmax {
            break;
        }
    }

    if possible_max_i >= possible_min_i {
        possible_min_i..(possible_max_i + 1)
    } else {
        0..0
    }
}

fn part1(tiles: &[IPoint]) -> isize {
    tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let dx = (a.x - b.x).abs();
            let dy = (a.y - b.y).abs();
            (dx + 1) * (dy + 1)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{Context, is_candidate_valid};
    use aoc::two::pt;

    #[test]
    fn simple_box() {
        // #xxxxx#xxxxx# (20, 0)
        // x...........x
        // x...........x
        // #xxxxx#xxxxx# (20, 10)
        let ctx = Context::new(vec![
            pt(0, 0),
            pt(10, 0),
            pt(20, 0),
            pt(20, 10),
            pt(10, 10),
            pt(0, 10),
        ]);
        assert!(is_candidate_valid(&ctx, pt(0, 0), pt(10, 0)));
        assert!(is_candidate_valid(&ctx, pt(0, 0), pt(0, 0)));
        assert!(is_candidate_valid(&ctx, pt(0, 0), pt(10, 10)));
        assert!(is_candidate_valid(&ctx, pt(0, 0), pt(20, 10)));
    }

    #[test]
    fn box_with_chunk_missing() {
        // #x#xx#xxxxx#
        // x.x..x.....x
        // x.#xx#.....x
        // x..........x
        // #xxxxx#xxxx#
        let ctx = Context::new(vec![
            pt(0, 0),
            pt(10, 0),
            pt(10, 3),
            pt(13, 3),
            pt(13, 0),
            pt(20, 0),
            pt(20, 10),
            pt(10, 10),
            pt(0, 10),
        ]);
        assert!(is_candidate_valid(&ctx, pt(0, 0), pt(10, 10)));
        assert!(is_candidate_valid(&ctx, pt(13, 0), pt(20, 10)));
        assert!(!is_candidate_valid(&ctx, pt(0, 0), pt(20, 10)));

        // Is 'outside', how do we know?
        // assert!(!is_candidate_valid(&ctx, pt(10, 0), pt(13, 3)));
    }
}
