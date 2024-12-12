use std::collections::{HashMap, VecDeque};

use aoc::{
    lines,
    two::{pt, DenseField, Dirn, Point, DOWN, RIGHT},
};

fn conv(value: u8) -> usize {
    match value {
        b'0'..=b'9' => (value as char).to_digit(10).unwrap() as usize,
        _ => panic!(),
    }
}

fn main() {
    let input = lines("aoc2023/input/day17");
    let field = DenseField::from_lines_with(input, conv);

    let part1 = search(&field, 0, 3);
    let part2 = search(&field, 4, 10);
    dbg!(part1);
    dbg!(part2);
}

fn search(field: &DenseField<usize>, min: u32, max: u32) -> usize {
    let mut q = VecDeque::new();
    let mut seen = HashMap::<(Point<isize>, Point<isize>, u32), usize>::new();

    let mut min_loss_at_exit = usize::MAX;

    // Start top left, no momentum.
    q.push_back((pt(0, 0), RIGHT, 0, 0_usize));
    q.push_back((pt(0, 0), DOWN, 0, 0_usize));

    while let Some((p, dir, momentum, loss)) = q.pop_back() {
        if let Some(&prev_loss) = seen.get(&(p, dir, momentum)) {
            // Already seen this and got there faster!
            if prev_loss <= loss {
                continue;
            }

            // No point exploring this route if it's already worse than a
            // complete one we've seen.
            if loss >= min_loss_at_exit {
                continue;
            }
        }

        seen.insert((p, dir, momentum), loss);

        if p == pt(field.width() - 1, field.height() - 1) && momentum >= min {
            min_loss_at_exit = min_loss_at_exit.min(loss);
            continue;
        }

        for next_dir in Dirn::all() {
            if next_dir == -dir {
                // can't go backwards
                continue;
            }

            // If we change direction, momentum will be 1 at the new location.
            let next_momentum = if dir == next_dir { momentum + 1 } else { 1 };
            let next_p = p + next_dir;
            let next_loss = loss
                + if let Some(cell) = field.try_get(next_p) {
                    cell
                } else {
                    continue; // this isn't on the field.
                };

            if next_dir == dir {
                if next_momentum <= max {
                    q.push_back((next_p, next_dir, next_momentum, next_loss))
                }
            } else if momentum >= min {
                q.push_back((next_p, next_dir, next_momentum, next_loss))
            }
        }
    }

    min_loss_at_exit
}
