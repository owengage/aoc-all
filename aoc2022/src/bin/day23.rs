use aoc::{fetch_input, text};
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::Add;

fn main() {
    // Field is infinite, so do not want to model it as an actual grid in memory.
    let mut field = parse(&text(fetch_input(2022, 23)));
    let mut directions = make_directions();
    let neighbour_deltas = make_surroundings();
    assert_eq!(8, neighbour_deltas.len());

    for i in 0.. {
        let starting_elves = field.clone();

        // First half
        let mut proposals = FxHashMap::default(); // k: proposed, v: which elf
        let mut conflicts = FxHashSet::default();

        for &p in &field {
            if !contains_elf(&field, p, &neighbour_deltas) {
                // Elf stays still if it has no neighbours.
                continue;
            }

            // It does have neighbours somewhere, check the directions one at a time
            // to try and propose a move.
            for dir in &directions {
                if !contains_elf(&field, p, dir) {
                    // Propose moving that direction, which will be the middle of
                    // the direction. If there is already a proposal to move
                    // there, remove the existing proposal, don't add this one,
                    // and add it to a set of conflicts to make sure no other
                    // elf tries too.
                    let prop = p + dir[1];
                    if !conflicts.contains(&prop) && proposals.insert(prop, p).is_some() {
                        proposals.remove(&prop);
                        conflicts.insert(prop);
                    }
                    break; // found our direction.
                }
            }
        }

        // Second half
        // We have already de-conflicted the elf movements, so all the remaining
        // proposals should just be simple moves.
        for (prop, elf_p) in proposals {
            field.remove(&elf_p);
            field.insert(prop);
        }

        // Finally, the directions shift one.
        directions.rotate_left(1);

        // println!("Round: {}", i + 1);
        // print_field(&field);

        if i == 9 {
            let (min, max) = rect_bounds(&field);
            let size = (max.x - min.x + 1) * (max.y - min.y + 1);

            println!("Part 1: {}", size as usize - field.len());
        }

        // Did any elves move?
        if starting_elves == field {
            println!("No elves moved in round {}", i + 1);
            break;
        }
    }
}

fn rect_bounds<'p>(field: impl IntoIterator<Item = &'p Point>) -> (Point, Point) {
    field.into_iter().fold(
        (
            Point::new(isize::MAX, isize::MAX),
            Point::new(isize::MIN, isize::MIN),
        ),
        |(min, max), p| {
            (
                Point::new(min.x.min(p.x), min.y.min(p.y)),
                Point::new(max.x.max(p.x), max.y.max(p.y)),
            )
        },
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

fn contains_elf<'dir, const N: usize>(
    field: &FxHashSet<Point>,
    p: Point,
    deltas: &[Point; N],
) -> bool {
    deltas.into_iter().any(|&delta| {
        let neighbour = p + delta;
        field.contains(&neighbour)
    })
}

fn make_directions() -> [[Point; 3]; 4] {
    [
        [Point::new(-1, -1), Point::new(0, -1), Point::new(1, -1)], // North
        [Point::new(-1, 1), Point::new(0, 1), Point::new(1, 1)],    // South
        [Point::new(-1, -1), Point::new(-1, 0), Point::new(-1, 1)], // West
        [Point::new(1, -1), Point::new(1, 0), Point::new(1, 1)],    // East
    ]
}

fn make_surroundings() -> [Point; 8] {
    [
        Point::new(-1, -1),
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(-1, 0),
        Point::new(-1, 1),
        Point::new(1, -1),
        Point::new(1, 0),
        Point::new(1, 1),
    ]
}

fn parse(input: &str) -> FxHashSet<Point> {
    let mut n = 0;

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        n = (n + 1) % 10;
                        Some(Point::new(x as isize, y as isize))
                    } else {
                        None
                    }
                })
                .collect::<FxHashSet<Point>>()
        })
        .collect()
}
