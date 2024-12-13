use aoc::{fetch_input, line_blocks};
use itertools::Itertools;
use regex::Regex;

struct Game {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    targetx: f64,
    targety: f64,
}

fn main() {
    let games = line_blocks(fetch_input(2024, 13));
    let games = games.into_iter().map(|g| parse_game(&g)).collect_vec();

    let solves = games.iter().filter_map(solve_ab1).collect_vec();
    let part1: isize = solves.iter().map(|(a, b)| a * 3 + b).sum();
    dbg!(part1);

    let part2_shift = 10000000000000f64;
    let games = games
        .into_iter()
        .map(|mut g| {
            g.targetx += part2_shift;
            g.targety += part2_shift;
            g
        })
        .collect_vec();
    let solves = games.iter().filter_map(solve_ab2).collect_vec();
    let part2: isize = solves.iter().map(|(a, b)| a * 3 + b).sum();
    dbg!(part2);
}

fn solve_ab1(g: &Game) -> Option<(isize, isize)> {
    let anumerator = g.targetx - (g.targety * g.bx) / g.by;
    let adenom = g.ax - (g.ay * g.bx) / g.by;
    let a = (anumerator / adenom).round() as isize;
    let b = ((g.targetx - ((anumerator * g.ax) / adenom)) / g.bx).round() as isize;

    if g.targetx as isize != a * g.ax as isize + b * g.bx as isize {
        return None;
    }
    if g.targety as isize != a * g.ay as isize + b * g.by as isize {
        return None;
    }
    if (0..=100).contains(&a) && (0..=100).contains(&b) {
        Some((a, b))
    } else {
        None
    }
}

fn solve_ab2(g: &Game) -> Option<(isize, isize)> {
    let anumerator = g.targetx - (g.targety * g.bx) / g.by;
    let adenom = g.ax - (g.ay * g.bx) / g.by;
    let a = (anumerator / adenom).round() as isize;
    let b = ((g.targetx - ((anumerator * g.ax) / adenom)) / g.bx).round() as isize;

    if g.targetx as isize != a * g.ax as isize + b * g.bx as isize {
        return None;
    }
    if g.targety as isize != a * g.ay as isize + b * g.by as isize {
        return None;
    }
    Some((a, b))
}

fn parse_game(lines: &[String]) -> Game {
    let button_re = Regex::new(r#"Button [AB]: X\+(\d+), Y\+(\d+)"#).unwrap();
    let prize_re = Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap();

    let acaps = button_re.captures(&lines[0]).unwrap();
    let ax = acaps.get(1).unwrap().as_str().parse().unwrap();
    let ay = acaps.get(2).unwrap().as_str().parse().unwrap();

    let bcaps = button_re.captures(&lines[1]).unwrap();
    let bx = bcaps.get(1).unwrap().as_str().parse().unwrap();
    let by = bcaps.get(2).unwrap().as_str().parse().unwrap();

    let pcaps = prize_re.captures(&lines[2]).unwrap();
    let targetx = pcaps.get(1).unwrap().as_str().parse().unwrap();
    let targety = pcaps.get(2).unwrap().as_str().parse().unwrap();
    Game {
        ax,
        ay,
        bx,
        by,
        targetx,
        targety,
    }
}

#[cfg(test)]
mod test {
    use aoc::lines_from_str;

    use crate::*;

    #[test]
    fn test1() {
        let game = parse_game(&lines_from_str(
            r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
        "#
            .trim(),
        ));

        let solve = solve_ab1(&game);
        println!("{:?}", solve);
    }

    #[test]
    fn test2() {
        let game = parse_game(&lines_from_str(
            r#"
Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450
        "#
            .trim(),
        ));

        let solve = solve_ab1(&game);
        println!("{:?}", solve);
    }
}
