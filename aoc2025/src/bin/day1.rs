use aoc::{fetch_input, lines};

fn main() {
    let input = lines(fetch_input(2025, 1));
    let mut dial = 50; // starts at 50.
    let mut part1 = 0;
    let mut part2 = 0;

    for mut line in input {
        let mut num: isize = line.split_off(1).parse().unwrap();
        let dir = line;

        part2 += num / 100;
        num = num % 100;

        if dial != 0 && num != 0 {
            match dir.as_str() {
                "L" => {
                    if dial - num <= 0 {
                        part2 += 1
                    }
                }
                "R" => {
                    if dial + num >= 100 {
                        part2 += 1;
                    }
                }
                _ => panic!(),
            }
        }

        // Update the dial
        match dir.as_str() {
            "L" => dial -= num,
            "R" => dial += num,
            _ => panic!("{}, {}", dir, num),
        }

        // We know dial can only be [-99,198] since we modulo'd `num`.
        dial = dial.rem_euclid(100);

        part1 += (dial == 0) as usize;
    }

    println!("part1 = {}", part1);
    println!("part2 = {}", part2);

    assert_eq!(part1, 1092);
    assert_eq!(part2, 6616);
}
