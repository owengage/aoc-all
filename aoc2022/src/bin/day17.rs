#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Air,
    Rock,
    Falling,
}

use Cell::*;

fn r(row: &str) -> [Cell; 7] {
    assert_eq!(row.len(), 7);
    let rock: Vec<_> = row
        .chars()
        .map(|c| match c {
            '#' => Cell::Falling,
            '.' => Cell::Air,
            _ => panic!(),
        })
        .collect();

    rock.try_into().unwrap()
}

fn br(row: &str) -> [Cell; 7] {
    assert_eq!(row.len(), 7);
    let rock: Vec<_> = row
        .chars()
        .map(|c| match c {
            '#' => Cell::Rock,
            '.' => Cell::Air,
            _ => panic!(),
        })
        .collect();

    rock.try_into().unwrap()
}

fn main() {
    let dirs: Vec<_> = std::fs::read_to_string("input.txt")
        .unwrap()
        // let dirs: Vec<_> = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            c => panic!("{:?}", c),
        })
        .collect();

    let mut dirs = dirs.iter().cycle();

    // Start with rock floor for simplicity.
    // let mut grid = vec![[Cell::Rock; 7]; 1];
    let mut grid = vec![br("#######"), br(".#...#."), br(".....#.")];

    //
    // Repeating section looks like
    // .....#.
    // .#...#.
    // #######
    //

    let air_row = r(".......");

    let rocks = [
        vec![r("..####.")],
        vec![r("...#..."), r("..###.."), r("...#...")],
        vec![r("..###.."), r("....#.."), r("....#..")],
        vec![r("..#...."); 4],
        vec![r("..##..."), r("..##...")],
    ];

    let mut pile_height = 3; // n means n rocks tall, so n-1 index.

    // n=1, 77355
    // n=2, 154691
    // n=3, 231958
    // n=4, 309302, ,,, 309,382
    // n=5, 386566
    // n=6, 463910
    // n=7, 541174
    // n=8, 618514
    // n=9, 695790
    // n=10, 773127
    // n=12, 927729
    // n=14, 1082345
    // n=15, 1159630

    // n=25, 1932707

    // n=16, 1236951
    // n=32, 2473819
    // n=64, 4947595
    // n=128, 9895153
    // let n = std::env::var("N").unwrap().parse::<usize>().unwrap();

    for (_, rock) in rocks.iter().cycle().take(921535).enumerate() {
        // How deep until each column has some blocker?
        // let mut cover = [Air; 7];
        // let mut depth = 0;
        // while cover != [Rock; 7] {
        //     depth += 1;
        //     let layer = grid[pile_height - depth];
        //     for (l, c) in layer.iter().zip(&mut cover) {
        //         if *l == Rock {
        //             *c = Rock;
        //         }
        //     }
        // }
        // if depth == 3 && (count % (5 * dir_count) == 0) {
        //     println!(
        //         "{}: Needed depth: {} (height: {})",
        //         count, depth, pile_height
        //     );
        //     print_grid(&grid[pile_height - depth..pile_height]);
        // }

        let mut rock = rock.clone();
        let rock_height = rock.len();
        let min_len = rock_height + pile_height + 3;

        // make sure we have space to place the rock.
        if grid.len() < min_len {
            grid.resize(min_len, air_row);
        }

        for i in (0..min_len).rev() {
            //jet
            let dir = dirs.next().unwrap();
            let old_rock = rock.clone();
            match dir {
                Dir::Left => rotate(&mut rock, -1),
                Dir::Right => rotate(&mut rock, 1),
            };

            // Has the rotation caused is to collide?

            let rotate_check = &grid[i + 1 - rock_height..i + 1];
            let collision_rotate = is_collision(rotate_check, &rock);

            // If would cause collison by rotating, don't rotate.
            if collision_rotate {
                rock = old_rock;
            }

            // fall
            // is there room below?
            let below_check = &grid[i - rock_height..i];
            let collision_below = is_collision(below_check, &rock);

            if collision_below {
                // solidify rock at current location.
                add_rock(&mut grid, &rock, i, Rock);
                pile_height = pile_height.max(i + 1);
                break; // next rock
            }
            // solidify?
            // if cannot fall, becomes rock at previous height.
        }

        // Simulate fall...

        // We need to know that the entire rock could move down one before we
        // actually mutate the grid.

        // Can we fall?
        for layer_i in (0..rock_height).rev() {
            let rock_bottom = grid[min_len - layer_i - 1];
            let below_rock = grid[min_len - layer_i - 2];

            let clear = rock_bottom.into_iter().zip(below_rock).all(|(r, b)| {
                if r == Falling {
                    b == Air
                } else {
                    true
                }
            });

            // We're clear, so can drop the rock one.
            if clear {
                for (i, r) in rock_bottom.iter().enumerate() {
                    if *r == Falling {
                        grid[min_len - rock_height - 1][i] = Falling;
                        grid[min_len - rock_height][i] = Air;
                    }
                }
            } else {
                // Solidify the rock.
            }
        }
    }

    // print_grid(&grid[grid.len() - 20..]);
    println!("{}", pile_height - 1);

    // Lets grow the field 'down' so that we have unbounded room.
}

fn rotate(rock: &mut [[Cell; 7]], mut d: isize) {
    // Check there's room to move the rock
    // Move the rock maximum amount up to d.
    while d != 0 {
        if d > 0 {
            d -= 1;
            // move things to the right, need to check ends free.
            if rock.iter().all(|row| row[6] == Air) {
                rock.iter_mut().for_each(|row| row.rotate_right(1))
            }
        }
        if d < 0 {
            d += 1;
            if rock.iter().all(|row| row[0] == Air) {
                rock.iter_mut().for_each(|row| row.rotate_left(1))
            }
        }
    }
}

fn add_rock(grid: &mut [[Cell; 7]], rock: &[[Cell; 7]], top_i: usize, t: Cell) {
    for (j, layer) in rock.iter().enumerate() {
        let g = &mut grid[top_i - rock.len() + j + 1];
        for (g, &c) in g.iter_mut().zip(layer) {
            if c == Falling {
                assert_eq!(*g, Air);
                *g = t;
            }
        }
    }
}

fn is_collision(area_to_check: &[[Cell; 7]], rock: &[[Cell; 7]]) -> bool {
    assert_eq!(area_to_check.len(), rock.len());

    area_to_check
        .iter()
        .flatten()
        .zip(rock.iter().flatten())
        .any(|(&a, &r)| a == Rock && r == Falling)
}
