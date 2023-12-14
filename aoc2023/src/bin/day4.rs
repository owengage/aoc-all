use std::collections::HashSet;

use aoc::lines;

fn main() {
    let mut games: Vec<Game> = lines("input/work4").into_iter().map(parse_game).collect();

    let win_counts: Vec<usize> = games
        .iter()
        .filter_map(|g| (g.win_count > 0).then_some(g.win_count))
        .collect();

    let part1: usize = win_counts.iter().map(|c| 2usize.pow(*c as u32 - 1)).sum();
    dbg!(part1);

    for i in 0..games.len() {
        let win_count = games[i].win_count;
        let instances = games[i].instances;
        for j in 0..win_count {
            // Each win_count cards after this one each get new copies
            games[i + j + 1].instances += instances;
        }
    }
    let part2: usize = games.iter().map(|g| g.instances).sum();
    dbg!(part2);
}

#[derive(Debug)]
struct Game {
    instances: usize,
    win_count: usize,
}

fn parse_game(line: String) -> Game {
    let (_, rest) = line.split_once(':').unwrap();
    let (target, options) = rest.split_once('|').unwrap();
    let target: Vec<usize> = target
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let options: HashSet<usize> = options
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let win_count = target.iter().filter(|&t| options.contains(t)).count();

    Game {
        win_count,
        instances: 1,
    }
}
