use aoc::{fetch_input, text};
use core::str;

enum State {
    Nonsense,
    ParsingLeftOp,
    ParsingEnd(u64),
}

fn main() {
    let input = text(fetch_input(2024, 3)).into_bytes();
    let mut curr = 0;
    let mut state = State::Nonsense;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut enabled = true;

    while curr < input.len() {
        match state {
            State::Nonsense => {
                if is_literal(curr, b"mul(", &input) {
                    curr += 4;
                    state = State::ParsingLeftOp;
                } else if is_literal(curr, b"do()", &input) {
                    curr += 4;
                    enabled = true;
                } else if is_literal(curr, b"don't()", &input) {
                    curr += 7;
                    enabled = false;
                } else {
                    curr += 1;
                }
            }
            State::ParsingLeftOp => {
                if let Some((end, val)) = parse_int(&input, curr) {
                    curr = end;
                    if input[curr] == b',' {
                        curr += 1;
                        state = State::ParsingEnd(val);
                        continue;
                    }
                }
                state = State::Nonsense;
            }
            State::ParsingEnd(num1) => {
                if let Some((end, val)) = parse_int(&input, curr) {
                    curr = end;
                    if input[curr] == b')' {
                        curr += 1;
                        if enabled {
                            part2 += num1 * val;
                        }
                        part1 += num1 * val;
                    }
                }
                state = State::Nonsense;
            }
        }
    }

    dbg!(part1);
    dbg!(part2);
    // assert_eq!(82733683, part2);
}

fn parse_int(input: &[u8], mut curr: usize) -> Option<(usize, u64)> {
    let start = curr;

    loop {
        if input[curr].is_ascii_digit() {
            curr += 1;
        } else {
            return if curr > start {
                let num1: u64 = str::from_utf8(&input[start..curr])
                    .unwrap()
                    .parse()
                    .unwrap_or(u64::MAX); // value was too large.

                if num1 < 1000 {
                    Some((curr, num1))
                } else {
                    None
                }
            } else {
                None
            };
        }
    }
}

fn is_literal(curr: usize, literal: &[u8], input: &[u8]) -> bool {
    curr + literal.len() <= input.len() && &input[curr..curr + literal.len()] == literal
}
