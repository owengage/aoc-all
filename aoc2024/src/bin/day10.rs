use std::collections::{HashSet, VecDeque};

use aoc::{
    fetch_input, lines,
    two::{DenseField, IPoint},
};

fn main() {
    let field = DenseField::<u32>::from_lines_with(lines(fetch_input(2024, 10)), |c| {
        (c as char).to_digit(10).unwrap()
    });

    let heads = get_heads(&field);
    let mut part1 = 0;
    let mut part2 = 0;

    for head in heads {
        let mut trailheads = HashSet::<IPoint>::new();
        let mut q = VecDeque::new();
        q.push_back(head);

        while let Some(head) = q.pop_front() {
            // For each head, look around, add any valid next heads to queue.
            let val = *field.get(head);
            let target = val + 1;

            // We win!
            if val == 9 {
                part2 += 1;
                trailheads.insert(head);
                continue;
            }

            for (&nval, np) in field.neighbours4_bounded(head) {
                if nval == target {
                    q.push_back(np);
                }
            }
        }

        part1 += trailheads.len();
    }

    println!("part1 = {part1}");
    println!("part2 = {part2}");
}

fn get_heads(field: &DenseField<u32>) -> Vec<IPoint> {
    field.points().filter(|p| *field.get(*p) == 0).collect()
}
