use core::panic;
use std::collections::{HashMap, VecDeque};

use aoc::{
    fetch_input, lines,
    two::{DenseField, IPoint},
};

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Start,
    End,
    Wall,
    Empty,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Cell::Wall,
            b'S' => Cell::Start,
            b'E' => Cell::End,
            b'.' => Cell::Empty,
            _ => panic!(),
        }
    }
}

struct Head {
    p: IPoint,
    time: usize,
}

fn main() {
    // Do not use Dijkstra, as we are going to have to revisit nodes to figure
    // out when best to cheat. DFS/BFS is probably the way.
    let field = DenseField::<Cell>::from_lines(lines(fetch_input(2024, 20)));
    let start = field.find(&Cell::Start).unwrap();
    let end = field.find(&Cell::End).unwrap();

    let time_to_end = find_times(&field, start);
    let time_to_start = find_times(&field, end);
    let no_cheat_time = *time_to_end.get(end);
    assert_eq!(9432, no_cheat_time);

    let part1 = count_cheats(&time_to_start, &time_to_end, no_cheat_time, 2);
    println!("part1 = {part1}");

    let part2 = count_cheats(&time_to_start, &time_to_end, no_cheat_time, 20);
    println!("part2 = {part2}");
}

fn count_cheats(
    time_to_start: &DenseField<usize>,
    time_to_end: &DenseField<usize>,
    no_cheat_time: usize,
    max_cheat: usize,
) -> usize {
    let part2: usize = find_cheats(time_to_start, time_to_end, max_cheat)
        .into_iter()
        .map(|(time, count)| {
            let time_saved = no_cheat_time.saturating_sub(time);
            if time_saved >= 100 {
                count
            } else {
                0
            }
        })
        .sum();
    part2
}

fn find_cheats(
    time_to_start: &DenseField<usize>,
    time_to_end: &DenseField<usize>,
    max_cheat: usize,
) -> HashMap<usize, usize> {
    let mut cheats = HashMap::new();

    for cheat_start in time_to_start.points() {
        for cheat_end in time_to_start.points() {
            let cheat_time = cheat_start.taxicab_dist(cheat_end);
            if cheat_time > max_cheat {
                continue;
            }

            let to_start = *time_to_start.get(cheat_start);
            let to_end = *time_to_end.get(cheat_end);

            // start/end must be reachable without cheats.
            if to_start == usize::MAX || to_end == usize::MAX {
                continue;
            }

            let total_time = to_start + cheat_time + to_end;

            // Increment count of how much we've saved this much time.
            *cheats.entry(total_time).or_default() += 1;
        }
    }

    cheats
}

fn find_times(field: &DenseField<Cell>, start: IPoint) -> DenseField<usize> {
    let mut times = DenseField::new(field.width(), field.height(), usize::MAX);
    *times.get_mut(start) = 0;

    let mut q = VecDeque::new();
    q.push_back(Head { p: start, time: 0 });

    while let Some(head) = q.pop_back() {
        for (nc, np) in field.neighbours4_bounded(head.p) {
            match nc {
                Cell::Start | Cell::End | Cell::Empty => {
                    // we can go here.
                    if head.time + 1 < *times.get(np) {
                        q.push_back(Head {
                            p: np,
                            time: head.time + 1,
                        });
                        *times.get_mut(np) = head.time + 1;
                    }
                }
                Cell::Wall => {}
            }
        }
    }

    times
}

#[cfg(test)]
mod test {
    use aoc::{
        lines_from_str,
        two::{pt, DenseField},
    };

    use crate::{find_cheats, find_times, Cell};

    #[test]
    fn test_parse() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        let field = DenseField::<Cell>::from_lines(lines_from_str(input));
        let start = field.find(&Cell::Start).unwrap();
        let end = field.find(&Cell::End).unwrap();

        let time_to_end = find_times(&field, start);
        let time_to_start = find_times(&field, end);
        let no_cheat_time = *time_to_end.get(end);

        assert_eq!(84, no_cheat_time);

        for y in 0..field.height() {
            for x in 0..field.width() {
                let val = *time_to_start.get(pt(x, y));
                if val == usize::MAX {
                    print!(" ###");
                } else if val > 999 {
                    print!(" ...");
                } else {
                    print!("{:4}", val);
                }
            }
            println!()
        }

        for y in 0..field.height() {
            for x in 0..field.width() {
                let val = *time_to_end.get(pt(x, y));
                if val == usize::MAX {
                    print!(" ###");
                } else if val > 999 {
                    print!(" ...");
                } else {
                    print!("{:4}", val);
                }
            }
            println!()
        }

        let _: usize = find_cheats(&time_to_start, &time_to_end, 2)
            .into_iter()
            .map(|(time, count)| {
                let time_saved = no_cheat_time.saturating_sub(time);
                if time_saved >= 1 {
                    println!("{count} ways to save {time_saved}");
                    count
                } else {
                    0
                }
            })
            .sum();
    }
}
