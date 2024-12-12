use aoc::lines;
use std::collections::HashMap;

type Game = HashMap<String, usize>;

fn main() {
    let input = lines("input/day2");
    let part1 = part1(&input);
    let part2 = part2(&input);

    dbg!(part1);
    dbg!(part2);
}

fn part1(input: &[String]) -> usize {
    // 145 too low
    let mut part1 = 0;

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let games: Vec<_> = input.iter().map(|s| parse_game(s)).collect();

    for (i, game) in games.iter().enumerate() {
        let id = i + 1;
        if *game.get("red").unwrap_or(&0) > max_red {
            continue;
        }
        if *game.get("green").unwrap_or(&0) > max_green {
            continue;
        }
        if *game.get("blue").unwrap_or(&0) > max_blue {
            continue;
        }
        part1 += id;
    }

    part1
}

fn part2(input: &[String]) -> usize {
    let mut part2 = 0;

    let games: Vec<_> = input.iter().map(|s| parse_game(s)).collect();

    for game in games {
        let mut power = 1;
        power *= *game.get("red").unwrap_or(&0);
        power *= *game.get("green").unwrap_or(&0);
        power *= *game.get("blue").unwrap_or(&0);

        part2 += power;
    }

    part2
}

fn parse_game(line: &str) -> Game {
    let (_, rest) = line.split_once(':').unwrap();
    let draws = rest.split([';', ',']);

    let mut game = HashMap::new();
    for draw in draws {
        let (n, color) = draw.trim().split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();
        let current = *game.entry(color.to_string()).or_insert(0);
        game.insert(color.to_string(), current.max(n));
    }

    game
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc::lines_from_str;

    #[test]
    fn test_parse_game() {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let input = lines_from_str(input);
        let game = parse_game(&input[0]);
        assert_eq!(4, game["blue"]);
        assert_eq!(1, game["red"]);
        assert_eq!(3, game["green"]);
    }

    #[test]
    fn test_1() {
        let res = part1(&lines_from_str("Game 1: 1 blue"));
        assert_eq!(1, res);

        let res = part1(&lines_from_str("Game 1: 15 blue\nGame 2: 14 blue"));
        assert_eq!(2, res);
    }
}
