use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    usize,
};

use aoc::{
    fetch_input, lines,
    two::{pt, DenseField, Dirn, IPoint},
};

#[derive(Debug, Clone, PartialEq)]
enum CellValue {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Debug, Clone, PartialEq)]
struct Cell {
    lowest: [usize; 4],
    value: CellValue,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    head: IPoint,
    dirn: Dirn,
    score: usize,
    route: Vec<IPoint>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        Cell {
            value: match value {
                b'.' => CellValue::Empty,
                b'#' => CellValue::Wall,
                b'S' => CellValue::Start,
                b'E' => CellValue::End,
                _ => panic!(),
            },
            lowest: [usize::MAX; 4],
        }
    }
}

fn main() {
    let input = lines(fetch_input(2024, 16));
    let (field, best_cells) = do_it(input);

    let part1 = field
        .data()
        .iter()
        .find(|c| matches!(c.value, CellValue::End))
        .unwrap()
        .lowest
        .iter()
        .min()
        .unwrap();

    print_field(&field);
    print_best(&field, &best_cells);

    println!("part1 = {part1}");
    println!("part2 = {}", best_cells.len());
}

fn do_it(input: Vec<String>) -> (DenseField<Cell>, HashSet<IPoint>) {
    let mut field = DenseField::<Cell>::from_lines(input);
    let start = field
        .points()
        .find(|p| matches!(field.get(*p).value, CellValue::Start))
        .unwrap();

    // let mut heads = VecDeque::new();
    let mut heads = BinaryHeap::new();
    heads.push(State {
        head: start,
        dirn: Dirn::Right,
        score: 0,
        route: Vec::new(),
    });

    // start east
    let mut best_end = usize::MAX;
    let mut best_cells = HashSet::new();

    // Depth first, straight line first.
    while let Some(State {
        head,
        dirn,
        score,
        mut route,
    }) = heads.pop()
    {
        // println!("Depth: {}", heads.len());
        println!("{:?}, {:?}, {:?}", head, dirn, score);
        // print_field(&field);

        // Update score for where we are.
        {
            let headcell = field.get_mut(head);
            let lowest = &mut headcell.lowest;
            let lowest = &mut lowest[dirn as usize];
            if score > *lowest || score > 82460 {
                // temp: use hard coded lowest, just faster.
                // println!("Skipping");
                continue;
            }
            if headcell.value == CellValue::End && score < *lowest {
                // If this is a new best score for the end, then clear the 'best
                // cells'.
                println!("Clearing best");
                best_cells.clear();
            }
            *lowest = score;
        }

        route.push(head);

        if field.get(head).value == CellValue::End {
            println!("End! {}", score);
            best_end = score;
            // Add ALL cells from this best route.
            best_cells.extend(route);
            continue;
        }

        let left = dirn.anticlockwise();
        if field.get(head + left.as_point()).value != CellValue::Wall {
            heads.push(State {
                head: head + left.as_point(),
                dirn: left,
                score: score + 1001,
                route: route.clone(),
            });
        }

        let right = dirn.clockwise();
        if field.get(head + right.as_point()).value != CellValue::Wall {
            heads.push(State {
                head: head + right.as_point(),
                dirn: right,
                score: score + 1001,
                route: route.clone(),
            });
        }

        let forward = head + dirn.as_point();
        let forward_cell = field.get(forward).clone();
        if forward_cell.value != CellValue::Wall {
            heads.push(State {
                head: forward,
                dirn,
                score: score + 1,
                route: route.clone(),
            });
        }
    }
    (field, best_cells)
}

fn print_field(field: &DenseField<Cell>) {
    for y in 0..field.height() {
        for x in 0..field.width() {
            let cell = field.get(pt(x, y));
            let ch = match cell.value {
                CellValue::Empty => {
                    if *cell.lowest.iter().min().unwrap() < usize::MAX {
                        'o'
                    } else {
                        '.'
                    }
                }
                CellValue::Wall => '#',
                CellValue::Start => 'S',
                CellValue::End => 'E',
            };
            print!("{}", ch)
        }
        println!();
    }
}

fn print_best(field: &DenseField<Cell>, best_cells: &HashSet<IPoint>) {
    for y in 0..field.height() {
        for x in 0..field.width() {
            let cell = field.get(pt(x, y));
            let ch = match cell.value {
                CellValue::Empty | CellValue::Start | CellValue::End => {
                    if best_cells.contains(&pt(x, y)) {
                        'o'
                    } else {
                        ' '
                    }
                }
                CellValue::Wall => '#',
            };
            print!("{}", ch)
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use aoc::lines_from_str;

    use crate::{do_it, print_best};

    #[test]
    fn test_parse() {
        let input = lines_from_str(
            r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#,
        );

        let (field, best_cells) = do_it(input);
        print_best(&field, &best_cells);
        println!("best {}", best_cells.len());
    }
}
