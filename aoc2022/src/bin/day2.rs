use aoc::{fetch_input, lines};

fn main() {
    // A B C rock paper scissors
    // part1: X Y Z rock paper scissors
    // part2: X Y Z win draw lose
    let input = lines(fetch_input(2022, 2));

    let games: Vec<_> = input
        .into_iter()
        .map(|line| {
            let mut it = line.split(' ').flat_map(str::parse::<char>);
            let elf = it.next().unwrap();
            let me = it.next().unwrap();
            (elf, me)
        })
        .collect();

    let part1 = games.iter().fold(0, |score, (elf, me)| match elf {
        'A' => match me {
            'X' => score + 1 + 3,
            'Y' => score + 2 + 6,
            'Z' => score + 3,
            _ => panic!(),
        },
        'B' => match me {
            'X' => score + 1,
            'Y' => score + 2 + 3,
            'Z' => score + 3 + 6,
            _ => panic!(),
        },
        'C' => match me {
            'X' => score + 1 + 6,
            'Y' => score + 2,
            'Z' => score + 3 + 3,
            _ => panic!(),
        },
        _ => panic!(),
    });

    // A B C rock paper scissors
    // part1: X Y Z rock paper scissors
    // part2: X Y Z win draw lose

    let part2 = games.iter().fold(0, |score, (elf, me)| match elf {
        'A' => match me {
            'Z' => score + 2 + 6,
            'Y' => score + 1 + 3,
            'X' => score + 3,
            _ => panic!(),
        },
        'B' => match me {
            'Z' => score + 3 + 6,
            'Y' => score + 2 + 3,
            'X' => score + 1,
            _ => panic!(),
        },
        'C' => match me {
            'Z' => score + 1 + 6,
            'Y' => score + 3 + 3,
            'X' => score + 2,
            _ => panic!(),
        },
        _ => panic!(),
    });

    dbg!(part1);
    dbg!(part2);
}
