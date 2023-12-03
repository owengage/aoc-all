use aoc::lines;

fn main() {
    let input = lines("input/day1");
    let mut part1 = 0;
    let mut part2 = 0;

    for line in &input {
        let digits: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();
        let val = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        let val: u32 = val.parse().unwrap();
        part1 += val;
    }

    dbg!(part1);

    for line in &input {
        let digits: Vec<_> = find_digits(line);
        let val = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        let val: u32 = val.parse().unwrap();
        part2 += val;
    }

    dbg!(part2);
}

const WORDS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn find_digits(line: &str) -> Vec<char> {
    if !line.is_ascii() {
        panic!();
    }

    let line = line.as_bytes();
    let mut digits = vec![];

    for i in 0..line.len() {
        let sub = &line[i..];
        if sub[0].is_ascii_digit() {
            digits.push(char::from_u32(sub[0] as u32).unwrap());
            continue;
        }
        for (j, word) in WORDS.iter().enumerate() {
            if sub.starts_with(word) {
                digits.push(char::from_digit((j + 1) as u32, 10).unwrap());
                break;
            }
        }
    }

    digits
}
