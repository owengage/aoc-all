use core::panic;
use std::collections::HashSet;

use aoc::{
    fetch_input, lines,
    two::{DenseField, IPoint, DOWN, LEFT, RIGHT, UP},
};

fn main() {
    let original_field = DenseField::<u8>::from_lines(lines(fetch_input(2024, 6)));
    let original_guard = original_field.find(&b'^').unwrap();

    let mut field = original_field.clone();
    let mut guard = original_guard;
    let mut visited = HashSet::new();
    visited.insert(guard);

    loop {
        // Can we walk forward?
        let next_pt = guard + to_dirn(*field.get(guard.x, guard.y));
        let Some(next) = field.try_get(next_pt.x, next_pt.y) else {
            break; // fell off.
        };

        match next {
            b'.' => {
                // Can walk forward.
                let old = *field.get(guard.x, guard.y);
                *field.get_mut(next_pt.x, next_pt.y) = old;
                *field.get_mut(guard.x, guard.y) = b'.';

                guard = next_pt;
                visited.insert(guard);
            }
            b'#' => {
                // Can't walk forward, just rotate the guard in place for ease.
                let gval = field.get_mut(guard.x, guard.y);
                *gval = match gval {
                    b'^' => b'>',
                    b'>' => b'v',
                    b'v' => b'<',
                    b'<' => b'^',
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    println!("part1 = {}", visited.len());

    // This visited list is now the list of places worth trying to put an
    // obstacle. Once we do place an obstacle we need to see if this leads to a
    // loop or not. We can do this by tracking the set of point + direction. If
    // we end up with the same point + direction we know we're going to repeat
    // forever.
    assert!(visited.remove(&original_guard));
    let mut count = 0;

    for obstacle in visited {
        // Create field with new obstacle.
        let mut field = original_field.clone();
        *field.get_mut(obstacle.x, obstacle.y) = b'#';

        if contains_loop(original_guard, field) {
            count += 1;
        }
    }

    println!("part2 = {count}");
}

fn contains_loop(mut guard: IPoint, mut field: DenseField<u8>) -> bool {
    let mut history = HashSet::<(u8, IPoint)>::new(); // dirn, point.

    loop {
        // Can we walk forward?
        let next_pt = guard + to_dirn(*field.get(guard.x, guard.y));
        let Some(next) = field.try_get(next_pt.x, next_pt.y) else {
            return false; // fell off.
        };

        match next {
            b'.' => {
                // Can walk forward.
                let old = *field.get(guard.x, guard.y);
                *field.get_mut(next_pt.x, next_pt.y) = old;
                *field.get_mut(guard.x, guard.y) = b'.';

                guard = next_pt;
                if history.insert((old, guard)) {
                    // not been here
                } else {
                    return true; // been here!
                }
            }
            b'#' => {
                // Can't walk forward, just rotate the guard in place for ease.
                let gval = field.get_mut(guard.x, guard.y);
                *gval = match gval {
                    b'^' => b'>',
                    b'>' => b'v',
                    b'v' => b'<',
                    b'<' => b'^',
                    _ => panic!(),
                };

                // insert here
                if history.insert((*gval, guard)) {
                    // not been here
                } else {
                    return true; // been here!
                }
            }
            _ => panic!(),
        }
    }
}

fn to_dirn(ch: u8) -> IPoint {
    match ch {
        b'^' => UP,
        b'>' => RIGHT,
        b'<' => LEFT,
        b'v' => DOWN,
        _ => panic!(),
    }
}
