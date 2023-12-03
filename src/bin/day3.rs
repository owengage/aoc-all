use std::collections::HashMap;

use aoc::{
    lines,
    two::{BoundedField, Point},
};

fn main() {
    let input = lines("input/work3");
    let engine = BoundedField::<u8>::from_lines(input);

    let parts = get_parts(engine);

    // too high 6441017
    // too low   536993
    //           539590
    let part1 = parts.iter().map(|(_p, n)| n).sum::<usize>();
    dbg!(part1);

    let only_gears: Vec<_> = parts
        .into_iter()
        .filter_map(|(p, n)| (p.symbol == '*').then_some((p.p, n)))
        .collect();

    let mut adjacent = HashMap::<Point<isize>, Vec<usize>>::new();
    for (p, n) in only_gears {
        adjacent.entry(p).or_default().push(n);
    }

    let mut part2 = 0;
    for pl in adjacent {
        if pl.1.len() == 2 {
            part2 += pl.1.iter().product::<usize>();
        }
    }

    dbg!(part2);
}

#[derive(Debug)]
struct Part {
    symbol: char,
    p: Point<isize>,
}

fn get_parts(engine: BoundedField<u8>) -> Vec<(Part, usize)> {
    let mut parts = vec![];

    for y in 0..engine.height {
        let mut current_number = String::new();

        for x in 0..engine.width {
            let c = engine.get(x, y);
            match c {
                c if c.is_ascii_digit() => current_number.push(c as char),
                _ => {
                    if !current_number.is_empty() {
                        let n: usize = current_number.parse().unwrap();
                        let nlen = current_number.len();
                        current_number.clear();

                        // what symbol are we near, if any?
                        'outer: for i in (x - nlen as isize)..x {
                            for (neighbour, p) in engine.eight_neighbours(i, y) {
                                if !neighbour.is_ascii_digit() && neighbour != b'.' {
                                    let symbol = neighbour as char;
                                    parts.push((Part { symbol, p }, n));
                                    break 'outer;
                                }
                            }
                        }
                    }
                } // _symbol => {
                  //     // we're a symbol.
                  // }
            }
        }
        if !current_number.is_empty() {
            let n: usize = current_number.parse().unwrap();
            let nlen = current_number.len();
            current_number.clear();

            // what symbol are we near, if any?
            'outer: for i in (engine.width - nlen as isize)..engine.width {
                for (neighbour, p) in engine.eight_neighbours(i, y) {
                    if !neighbour.is_ascii_digit() && neighbour != b'.' {
                        let symbol = neighbour as char;
                        parts.push((Part { symbol, p }, n));
                        break 'outer;
                    }
                }
            }
        }
    }

    parts
}

#[cfg(test)]
mod test {
    use aoc::{lines_from_str, two::BoundedField};

    use crate::get_parts;

    #[test]
    fn test_sep_by_symbol() {
        let input = lines_from_str(r#"..100*300..."#);
        let engine = BoundedField::<u8>::from_lines(input);

        let parts = get_parts(engine);
        let mut total = 0;
        for part in parts {
            total += part.1;
            println!("{:?}: {}", part.0, part.1);
        }
    }

    #[test]
    fn test_diagonal() {
        let input = lines_from_str(
            r#"............
...500......
......*....."#,
        );
        let engine = BoundedField::<u8>::from_lines(input);

        let parts = get_parts(engine);
        let mut total = 0;
        for part in parts {
            total += part.1;
            println!("{:?}: {}", part.0, part.1);
        }
    }

    #[test]
    fn test_edge() {
        let input = lines_from_str(
            r#"............
..........50
......*....*"#,
        );
        let engine = BoundedField::<u8>::from_lines(input);

        let parts = get_parts(engine);
        let mut total = 0;
        for part in parts {
            total += part.1;
            println!("{:?}: {}", part.0, part.1);
        }
    }

    #[test]
    fn test_parse() {
        let input = lines_from_str(
            r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
        );
        let engine = BoundedField::<u8>::from_lines(input);

        let parts = get_parts(engine);
        let mut total = 0;
        for part in parts {
            total += part.1;
            println!("{:?}: {}", part.0, part.1);
        }

        assert_eq!(total, 4361);
    }
}