use core::panic;
use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use aoc::{
    fetch_input, lines,
    two::{pt, DenseField, IPoint},
};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Start,
    End {
        lowest: HashMap<Option<(IPoint, IPoint)>, usize>,
    },
    Wall,
    Empty {
        lowest: HashMap<Option<(IPoint, IPoint)>, usize>,
    },
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Cell::Wall,
            b'S' => Cell::Start,
            b'E' => Cell::End {
                lowest: Default::default(),
            },
            b'.' => Cell::Empty {
                lowest: Default::default(),
            },
            _ => panic!(),
        }
    }
}

struct Head {
    p: IPoint,
    time: usize,
    cheat: Option<(IPoint, IPoint)>,
}

fn main() {
    // Do not use Dijkstra, as we are going to have to revisit nodes to figure
    // out when best to cheat. DFS/BFS is probably the way.
    let field = DenseField::<Cell>::from_lines(lines(fetch_input(2024, 20)));
    let start = field.find(&Cell::Start).unwrap();
    let end = field
        .find(&Cell::End {
            lowest: Default::default(),
        })
        .unwrap();

    let no_cheat = fastest_time(
        field.clone(),
        Head {
            p: start,
            time: 0,
            // pretend we've already cheated to disable it this run.
            cheat: Some((pt(0, 0), pt(0, 0))),
        },
        usize::MAX,
    );
    let fastest_no_cheat = match no_cheat.get(end) {
        Cell::End { lowest } => *lowest.get(&Some((pt(0, 0), pt(0, 0)))).unwrap(),
        _ => panic!(),
    };

    assert_eq!(9432, fastest_no_cheat);

    dbg!(fastest_no_cheat);

    let cheats = fastest_time(
        field.clone(),
        Head {
            p: start,
            time: 0,
            cheat: None,
        },
        fastest_no_cheat - 100,
    );

    let counts = match cheats.get(end) {
        Cell::End { lowest } => lowest.iter().map(|(_, t)| fastest_no_cheat - t).counts(),
        _ => panic!(),
    };

    let counts = counts.into_iter().sorted_by_key(|p| p.0).collect_vec();

    println!(
        "part2 = {}",
        counts.into_iter().map(|kv| kv.1).sum::<usize>()
    );
}

fn fastest_time(mut field: DenseField<Cell>, start: Head, cutoff: usize) -> DenseField<Cell> {
    let mut q = VecDeque::new();
    q.push_back(start);

    while let Some(head) = q.pop_back() {
        let ns = field
            .neighbours4_bounded(head.p)
            .map(|(nc, np)| np)
            .collect_vec();

        if head.time > cutoff {
            // skip
            continue;
        }

        for np in ns {
            let nc = field.get_mut(np);

            match nc {
                Cell::Start => {} // don't go back to the start...
                Cell::End { lowest } => {
                    let low = lowest.entry(head.cheat).or_insert(usize::MAX);
                    if head.time + 1 < *low {
                        *low = head.time + 1;
                    }
                    println!("Found end {:?}, {}", head.cheat, *low);
                }
                Cell::Wall => {
                    if head.cheat.is_none() {
                        // We can potentially cheat here.
                        let cheat_start = head.p;
                        // From this neighbour we need to find the cheat end.
                        // This will be the neighbours of this point that are
                        // 'empty'.
                        let cheat_ns = field
                            .neighbours4_bounded(np)
                            .map(|(nc, np)| np)
                            .collect_vec();

                        for np in cheat_ns {
                            let nc = field.get_mut(np);
                            match nc {
                                Cell::End { lowest } => {
                                    let cheat = Some((cheat_start, np));
                                    let low = lowest.entry(cheat).or_insert(usize::MAX);
                                    if head.time + 2 < *low {
                                        *low = head.time + 2;
                                    }
                                    println!("Found cheat end {:?}, {}", cheat, *low);
                                }
                                Cell::Empty { lowest } => {
                                    // we can land here for our cheat.
                                    let cheat = Some((cheat_start, np));
                                    let low = lowest.entry(cheat).or_insert(usize::MAX);
                                    if head.time + 2 < *low {
                                        *low = head.time + 2;
                                    }
                                    q.push_back(Head {
                                        p: np,
                                        time: head.time + 2,
                                        cheat,
                                    });
                                }
                                _ => {} // anything other than a empty/end we don't care about.
                            }
                        }
                    }
                }
                Cell::Empty { lowest } => {
                    let low = lowest.entry(head.cheat).or_insert(usize::MAX);

                    if head.time + 1 < *low {
                        *low = head.time + 1;
                        q.push_back(Head {
                            p: np,
                            time: head.time + 1,
                            cheat: head.cheat,
                        });
                    }
                }
            }
        }
    }

    field
}

#[cfg(test)]
mod test {
    use aoc::{
        lines, lines_from_str,
        two::{pt, DenseField},
    };
    use itertools::Itertools;

    use crate::{fastest_time, Cell, Head};

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
        let end = field
            .find(&Cell::End {
                lowest: Default::default(),
            })
            .unwrap();

        let no_cheat = fastest_time(
            field.clone(),
            Head {
                p: start,
                time: 0,
                cheat: None,
            },
            84 - 2,
        );

        let counts = match no_cheat.get(end) {
            Cell::End { lowest } => lowest.iter().map(|(_, t)| 84 - t).counts(),
            _ => panic!(),
        };

        let counts = counts.into_iter().sorted_by_key(|p| p.0).collect_vec();

        println!("{:?}", counts);
        println!(
            "part2 = {}",
            counts.into_iter().map(|kv| kv.1).sum::<usize>()
        );
    }
}
