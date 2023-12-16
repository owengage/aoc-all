use aoc::lines;
use itertools::Itertools;

fn main() {
    let input = lines("aoc2023/input/day15").pop().unwrap();
    let parts = input.split(',').collect_vec();
    let part1: u64 = parts.iter().map(|s| hash(s)).sum();

    dbg!(part1);

    let part2 = part2(parts);
    dbg!(part2);
}

fn part2(parts: Vec<&str>) -> usize {
    const VAL: Vec<Lens> = vec![];
    let mut boxes: [Vec<Lens>; 256] = [VAL; 256];

    for instruction in parts {
        if let Some(label) = instruction.strip_suffix('-') {
            let bx = &mut boxes[hash(label) as usize];

            // remove lens
            if let Some((i, _)) = bx.iter().find_position(|l| l.label == label) {
                bx.remove(i);
            }
        } else {
            // add lens
            let (label, focal) = instruction.split_once('=').unwrap();
            let bx = &mut boxes[hash(label) as usize];

            if let Some((i, _)) = bx.iter().find_position(|l| l.label == label) {
                // Lens with label already in box, replace it.
                bx[i] = Lens {
                    label: label.to_string(),
                    focal: focal.parse().unwrap(),
                };
            } else {
                bx.push(Lens {
                    label: label.to_string(),
                    focal: focal.parse().unwrap(),
                })
            }
        }
    }

    focusing_power(&boxes)
}

fn focusing_power(boxes: &[Vec<Lens>]) -> usize {
    let mut power = 0;

    for (b, bx) in boxes.iter().enumerate() {
        for (l, lens) in bx.iter().enumerate() {
            power += (1 + b) * (1 + l) * lens.focal;
        }
    }

    power
}

struct Lens {
    label: String,
    focal: usize,
}

fn hash(input: &str) -> u64 {
    let mut current = 0;
    for ch in input.chars() {
        assert!(ch.is_ascii());
        let ord = ch as u64;
        current += ord;
        current *= 17;
        current %= 256;
    }

    current
}
