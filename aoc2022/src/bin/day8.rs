use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Cell {
    height: i32,
    surrounding: [i32; 4],
}

struct Grid {
    data: Vec<Cell>,
    width: usize,
}

impl Grid {
    fn height_at(&self, x: usize, y: usize) -> i32 {
        assert!(x < self.width);
        assert!(y < self.width);
        self.data[x + y * self.width].height
    }
}

fn parse(input: impl BufRead) -> Grid {
    let mut input = input.lines().flatten().peekable();
    let width = input.peek().unwrap().len();

    Grid {
        data: input
            .flat_map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|digit| Cell {
                height: digit as i32,
                surrounding: [0; 4],
            })
            .collect(),
        width,
    }
}

fn part1(grid: &mut Grid) {
    for y in 0..grid.width {
        let mut lmax = -1;
        let mut rmax = -1;

        for x in 0..grid.width {
            let left = &mut grid.data[x + y * grid.width];
            left.surrounding[3] = lmax;
            lmax = lmax.max(left.height);

            let right = &mut grid.data[(grid.width - 1 - x) + y * grid.width];
            right.surrounding[1] = rmax;
            rmax = rmax.max(right.height);
        }
    }

    for x in 0..grid.width {
        let mut tmax = -1;
        let mut bmax = -1;

        for y in 0..grid.width {
            let top = &mut grid.data[x + y * grid.width];
            top.surrounding[0] = tmax;
            tmax = tmax.max(top.height);

            let bottom = &mut grid.data[x + (grid.width - 1 - y) * grid.width];
            bottom.surrounding[2] = bmax;
            bmax = bmax.max(bottom.height);
        }
    }
}

fn part2(grid: &Grid) -> u32 {
    fn do_tree(grid: &Grid, xtree: usize, ytree: usize) -> u32 {
        if xtree == 50 && ytree == 14 {
            println!("test");
        }
        let tree_height = grid.height_at(xtree, ytree);
        let mut score = 1; // multiplicative identity.
        let mut count = 0; // can always see one tree

        // left
        for i in 1..=xtree {
            let x = xtree - i; // go rightwards
            if grid.height_at(x, ytree) < tree_height {
                count += 1;
            } else {
                count += 1;
                break;
            }
        }

        score *= count;
        count = 0;

        // right
        for i in (xtree + 1)..grid.width {
            let x = i; // go leftwards
            if grid.height_at(x, ytree) < tree_height {
                count += 1;
            } else {
                count += 1;
                break;
            }
        }

        score *= count;
        count = 0;

        // top
        for i in 1..=ytree {
            let y = ytree - i; // go upwards
            if grid.height_at(xtree, y) < tree_height {
                count += 1;
            } else {
                count += 1;
                break;
            }
        }

        score *= count;
        count = 0;

        // bottom
        for i in (ytree + 1)..grid.width {
            let y = i; // go leftwards
            if grid.height_at(xtree, y) < tree_height {
                count += 1;
            } else {
                count += 1;
                break;
            }
        }

        score *= count;

        score
    }

    let mut scores = vec![];
    for y in 0..grid.width {
        for x in 0..grid.width {
            scores.push(do_tree(grid, x, y));
        }
    }

    *scores.iter().max().unwrap()
}

impl Cell {
    fn is_visible(&self) -> bool {
        self.surrounding.iter().any(|sur| *sur < self.height)
    }
}

fn main() {
    let mut grid = parse(BufReader::new(File::open("input.txt").unwrap()));
    part1(&mut grid);

    let part1 = grid.data.iter().filter(|c| c.is_visible()).count();
    let part2 = part2(&grid);

    dbg!(part1);
    dbg!(part2);
    // 500850 too low
    // count hidden
}
