use std::{
    fmt::{Display, Write},
    io::{stdin, stdout},
};

use aoc::{
    fetch_input, line_blocks,
    two::{DenseField, Dirn, IPoint, DOWN, LEFT, RIGHT, UP},
};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Robot,
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'@' => Cell::Robot,
            b'#' => Cell::Wall,
            b'.' => Cell::Empty,
            b'O' => Cell::BoxLeft,
            b'[' => Cell::BoxLeft,
            b']' => Cell::BoxRight,
            _ => panic!(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Robot => '@',
            Cell::Wall => '#',
            Cell::BoxLeft => '[',
            Cell::BoxRight => ']',
            Cell::Empty => '.',
        })
    }
}

fn main() {
    let input = line_blocks(fetch_input(2024, 15));
    let field = DenseField::<Cell>::from_lines(input[0].clone());
    let moves = input[1]
        .iter()
        .flat_map(|l| l.chars().map(Dirn::from_arrow))
        .collect_vec();

    part1(field.clone(), moves.clone());

    let mut field = part2_field(&input[0]);
    let mut robot = field.find(&Cell::Robot).unwrap();

    // loop {
    //     field.debug_print();
    //     let mut buf = String::new();
    //     stdin().read_line(&mut buf).unwrap();
    //     let d = Dirn::from_arrow(match buf.trim().chars().next() {
    //         Some(d) => d,
    //         None => continue,
    //     });
    //     println!("Moving {d:?}");

    //     move_robot2(&mut field, &mut robot, d);
    //     field.debug_print();
    // }

    for dirn in moves {
        move_robot2(&mut field, &mut robot, dirn)
    }
    field.debug_print();

    // high: 1482526
    println!("part2 = {}", calc_score(&field));
}

fn move_robot2(field: &mut DenseField<Cell>, robot: &mut IPoint, dirn: Dirn) {
    assert_eq!(*field.get(*robot), Cell::Robot);
    let destp = *robot + dirn.as_point();
    let dest = field.get(destp).clone();

    match dest {
        Cell::Robot => panic!(),
        Cell::Wall => {} // can't do anything.
        Cell::BoxRight | Cell::BoxLeft => {
            let box_left = match dest {
                Cell::BoxLeft => destp,
                Cell::BoxRight => destp + LEFT,
                _ => panic!(),
            };
            if shove_box(field, box_left, dirn, false) {
                *field.get_mut(destp) = Cell::Robot;
                *field.get_mut(*robot) = Cell::Empty;
                *robot = destp;
            }
        }
        Cell::Empty => {
            // Just move.
            *field.get_mut(destp) = Cell::Robot;
            *field.get_mut(*robot) = Cell::Empty;
            *robot = destp;
        }
    }
}

fn shove_box(field: &mut DenseField<Cell>, box_left: IPoint, dirn: Dirn, dry: bool) -> bool {
    // We need to check the direct boxes we'll effect first, then move our box
    // if they're all fine.
    match dirn {
        Dirn::Right => {
            let mov = |field: &mut DenseField<Cell>| {
                if dry {
                    panic!();
                };
                *field.get_mut(box_left) = Cell::Empty;
                *field.get_mut(box_left + RIGHT) = Cell::BoxLeft;
                *field.get_mut(box_left + 2 * RIGHT) = Cell::BoxRight;
            };
            match field.get(box_left + 2 * RIGHT).clone() {
                Cell::Wall => false,
                Cell::BoxLeft => {
                    // Otherwise shove the box in the way if we can.
                    if shove_box(field, box_left + 2 * RIGHT, Dirn::Right, dry) {
                        mov(field);
                        true
                    } else {
                        false
                    }
                }
                Cell::Empty => {
                    mov(field);
                    true
                }
                _ => panic!(),
            }
        }
        Dirn::Left => {
            let mov = |field: &mut DenseField<Cell>| {
                if dry {
                    panic!();
                };
                *field.get_mut(box_left) = Cell::BoxRight;
                *field.get_mut(box_left + LEFT) = Cell::BoxLeft;
                *field.get_mut(box_left + RIGHT) = Cell::Empty;
            };

            match field.get(box_left + LEFT).clone() {
                Cell::Wall => false,
                Cell::BoxRight => {
                    if shove_box(field, box_left + 2 * LEFT, Dirn::Left, dry) {
                        mov(field);
                        true
                    } else {
                        false
                    }
                }
                Cell::Empty => {
                    mov(field);
                    true
                }
                _ => panic!(),
            }
        }
        Dirn::Up => {
            let mov = |field: &mut DenseField<Cell>| {
                if dry {
                    return;
                };
                *field.get_mut(box_left) = Cell::Empty;
                *field.get_mut(box_left + RIGHT) = Cell::Empty;
                *field.get_mut(box_left + UP) = Cell::BoxLeft;
                *field.get_mut(box_left + UP + RIGHT) = Cell::BoxRight;
            };

            let blocker_left = field.get(box_left + UP).clone();
            let blocker_right = field.get(box_left + UP + RIGHT).clone();

            match blocker_left {
                Cell::Robot => panic!(),
                Cell::Wall => return false,
                Cell::BoxLeft => {
                    // simple case, we're directly below another box, so just
                    // need to check this one.
                    assert_eq!(blocker_right, Cell::BoxRight);
                    if shove_box(field, box_left + UP, Dirn::Up, dry) {
                        mov(field);
                        return true;
                    } else {
                        return false;
                    }
                }
                Cell::BoxRight => {
                    // complex case, got to check other side too.
                    // ..[]??
                    // ...[]. moving up...
                    if shove_box(field, box_left + UP + LEFT, Dirn::Up, true) {
                        // can't move yet, right may be blocked.
                    } else {
                        return false;
                    }
                }
                Cell::Empty => {} // fine here, still need to look at other blocker.
            }

            match blocker_right {
                Cell::Robot => panic!(),
                Cell::Wall => false,
                Cell::BoxLeft => {
                    // ..??[]
                    // ...[]. moving up...
                    if shove_box(field, box_left + UP + RIGHT, Dirn::Up, dry) {
                        if *field.get(box_left + UP + LEFT) == Cell::BoxLeft {
                            assert!(shove_box(field, box_left + UP + LEFT, Dirn::Up, dry));
                        }
                        mov(field); // can finally move it
                        true
                    } else {
                        false
                    }
                }
                Cell::BoxRight => panic!(), // should have been dealt with above.
                Cell::Empty => {
                    if *field.get(box_left + UP + LEFT) == Cell::BoxLeft {
                        assert!(shove_box(field, box_left + UP + LEFT, Dirn::Up, dry));
                    }
                    mov(field);
                    true
                }
            }
        }
        Dirn::Down => {
            let mov = |field: &mut DenseField<Cell>| {
                if dry {
                    return;
                };
                *field.get_mut(box_left) = Cell::Empty;
                *field.get_mut(box_left + RIGHT) = Cell::Empty;
                *field.get_mut(box_left + DOWN) = Cell::BoxLeft;
                *field.get_mut(box_left + DOWN + RIGHT) = Cell::BoxRight;
            };

            let blocker_left = field.get(box_left + DOWN).clone();
            let blocker_right = field.get(box_left + DOWN + RIGHT).clone();

            match blocker_left {
                Cell::Robot => panic!(),
                Cell::Wall => return false,
                Cell::BoxLeft => {
                    // simple case, we're directly below another box, so just
                    // need to check this one.
                    assert_eq!(blocker_right, Cell::BoxRight);
                    if shove_box(field, box_left + DOWN, Dirn::Down, dry) {
                        mov(field);
                        return true;
                    } else {
                        return false;
                    }
                }
                Cell::BoxRight => {
                    // complex case, got to check other side too.
                    // ..[]??
                    // ...[]. moving up...
                    if shove_box(field, box_left + DOWN + LEFT, Dirn::Down, true) {
                        // can't move yet, right may be blocked.
                    } else {
                        return false;
                    }
                }
                Cell::Empty => {} // fine here, still need to look at other blocker.
            }

            match blocker_right {
                Cell::Robot => panic!(),
                Cell::Wall => false,
                Cell::BoxLeft => {
                    // ..??[]
                    // ...[]. moving up...
                    if shove_box(field, box_left + DOWN + RIGHT, Dirn::Down, dry) {
                        if *field.get(box_left + DOWN + LEFT) == Cell::BoxLeft {
                            assert!(shove_box(field, box_left + DOWN + LEFT, Dirn::Down, dry));
                        }
                        mov(field); // can finally move it
                        true
                    } else {
                        false
                    }
                }
                Cell::BoxRight => panic!(), // should have been dealt with above.
                Cell::Empty => {
                    // try to shove other side.
                    if *field.get(box_left + DOWN + LEFT) == Cell::BoxLeft {
                        assert!(shove_box(field, box_left + DOWN + LEFT, Dirn::Down, dry));
                    }
                    mov(field);
                    true
                }
            }
        }
    }
}

fn part2_field(input: &[String]) -> DenseField<Cell> {
    DenseField::<Cell>::from_lines(
        input
            .iter()
            .map(|l| {
                l.bytes()
                    .map(|b| match b {
                        b'#' => "##",
                        b'@' => "@.",
                        b'.' => "..",
                        b'O' => "[]",
                        _ => panic!(),
                    })
                    .join("")
            })
            .collect_vec(),
    )
}

fn part1(mut field: DenseField<Cell>, moves: Vec<Dirn>) {
    let mut robot = field.find(&Cell::Robot).unwrap();

    for dirn in moves {
        move_robot(&mut field, &mut robot, dirn);
    }

    println!("part1 = {}", calc_score(&field));
}

fn calc_score(field: &DenseField<Cell>) -> isize {
    let mut score = 0;
    for p in field.points() {
        let cell = field.get(p);
        if let Cell::BoxLeft = cell {
            score += p.x + p.y * 100;
        }
    }
    score
}

fn move_robot(field: &mut DenseField<Cell>, robot: &mut IPoint, dirn: Dirn) {
    assert_eq!(*field.get(*robot), Cell::Robot);
    let destp = *robot + dirn.as_point();
    let dest = field.get(destp).clone();

    match dest {
        Cell::Robot => panic!(),
        Cell::Wall => {} // can't do anything.
        Cell::BoxRight => panic!(),
        Cell::BoxLeft => {
            // Need to check for boxes until we find an empty space or a wall.
            // If wall do nothing, if space move everything over.
            let mut end = destp;
            while *field.get(end) == Cell::BoxLeft {
                end += dirn.as_point();
            }

            let endv = field.get(end).clone();
            match endv {
                Cell::BoxLeft => panic!(),
                Cell::BoxRight => panic!(),
                Cell::Robot => panic!(),
                Cell::Wall => {} // do nothing!
                Cell::Empty => {
                    // move all the boxes along, and the robot.
                    let delta = end - *robot;
                    let steps = (delta.x + delta.y).abs(); // only one will be a non-zero

                    *field.get_mut(*robot + dirn.as_point()) = Cell::Robot;
                    *field.get_mut(*robot) = Cell::Empty;

                    // @OOO...
                    // .@OOO..

                    // Move boxes
                    for i in 2..=steps {
                        let p = *robot + i * dirn.as_point();
                        *field.get_mut(p) = Cell::BoxLeft;
                    }

                    *robot += dirn.as_point();
                }
            }
        }
        Cell::Empty => {
            // Just move.
            *field.get_mut(destp) = Cell::Robot;
            *field.get_mut(*robot) = Cell::Empty;
            *robot = destp;
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::{stdin, stdout, Write};

    use aoc::{
        lines_from_str,
        two::{pt, Dirn},
    };

    use crate::{move_robot2, part2_field, Cell};

    #[test]
    fn test_parse() {
        // let mut field = part2_field(&[
        //     "######".to_string(),
        //     ".##@.".to_string(),
        //     "..[].".to_string(),
        //     ".[]..".to_string(),
        //     "...[]".to_string(),
        //     ".....".to_string(),
        //     "#####".to_string(),
        // ]);

        let mut field = part2_field(&[
            "######".to_string(),
            "......".to_string(),
            "..@O..".to_string(),
            "...O..".to_string(),
            "......".to_string(),
            "######".to_string(),
        ]);
        let moves = [Dirn::Right, Dirn::Right, Dirn::Up, Dirn::Right, Dirn::Down];
        let mut robot = field.find(&Cell::Robot).unwrap();

        for d in moves {
            println!("Moving {d:?}");
            move_robot2(&mut field, &mut robot, d);
            field.debug_print();
        }
    }
}
