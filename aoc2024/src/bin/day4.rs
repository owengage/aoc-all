use std::collections::HashSet;

use aoc::{
    fetch_input, lines,
    two::{pt, DenseField, IPoint},
};

fn main() {
    let field: DenseField<u8> = DenseField::from_lines(lines(fetch_input(2024, 4)));

    // left is x, right is s of xmas.
    let part1 = part1(&field).len();
    let part2 = part2(&field).len();

    dbg!(part1);
    dbg!(part2);
}

fn part2(field: &DenseField<u8>) -> HashSet<IPoint> {
    let mut xmases = HashSet::<IPoint>::new(); // point of central A.

    for p in field.points() {
        if field.get(p.x, p.y) == &b'A' {
            let neigh = [p + pt(1, 1), p + pt(-1, 1), p + pt(1, -1), p + pt(-1, -1)];
            let mut count = 0;

            for n in neigh {
                if let Some(&b'M') = field.try_get(n.x, n.y) {
                    let other = p - (n - p);
                    let val = field.try_get(other.x, other.y);

                    if let Some(b'S') = val {
                        count += 1;
                    }
                }
            }

            if count == 2 {
                xmases.insert(p);
            }
        }
    }

    xmases
}

fn part1(field: &DenseField<u8>) -> HashSet<(IPoint, IPoint)> {
    let mut xmases = HashSet::<(IPoint, IPoint)>::new();

    for p in field.points() {
        if field.get(p.x, p.y) == &b'X' {
            // Found the X, do any neighbours give us the M?
            let neigh = field.neighbours8_bounded(p.x, p.y);
            'neighs: for n in neigh {
                if field.get(n.1.x, n.1.y) == &b'M' {
                    // Found the M, we now have a direction to look in for the
                    // extra characters.
                    let dirn = n.1 - p;
                    let rest = [b'A', b'S'];
                    for (i, ch) in rest.iter().enumerate() {
                        let offset = dirn * (i + 2) as isize; // +1 is the M, +2 is the A...
                        let probe = p + offset;
                        let val = field.try_get(probe.x, probe.y);
                        if val.is_none() || val.unwrap() != ch {
                            continue 'neighs; // this direction isn't XMAS.
                        }
                    }
                    // if we got here, we found XMAS in this direction.
                    xmases.insert((p, p + (dirn * 4)));
                }
            }
        }
    }
    xmases
}
