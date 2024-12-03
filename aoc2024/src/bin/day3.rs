use core::str;

use aoc::{fetch_input, text};

enum State {
    Nonsense,
    Num1(usize, usize),      // (start index, end index)
    Num2(u64, usize, usize), // first num, start, end
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
                    state = State::Num1(curr, curr);
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
            State::Num1(start, end) => {
                if end - start > 3 {
                    state = State::Nonsense;
                    continue;
                }
                if input[curr].is_ascii_digit() {
                    curr += 1;
                    state = State::Num1(start, curr);
                } else if input[curr] == b',' {
                    let len = end - start;
                    if (1..=3).contains(&len) {
                        let num1: u64 =
                            str::from_utf8(&input[start..end]).unwrap().parse().unwrap();
                        curr += 1;
                        state = State::Num2(num1, curr, curr)
                    }
                } else {
                    state = State::Nonsense;
                    continue;
                }
            }
            State::Num2(num1, start, end) => {
                if end - start > 3 {
                    state = State::Nonsense;
                    continue;
                }
                if input[curr].is_ascii_digit() {
                    curr += 1;
                    state = State::Num2(num1, start, curr);
                } else if input[curr] == b')' {
                    let len = end - start;
                    if (1..=3).contains(&len) {
                        let num2: u64 =
                            str::from_utf8(&input[start..end]).unwrap().parse().unwrap();
                        curr += 1;
                        state = State::Nonsense;

                        if enabled {
                            part2 += num1 * num2;
                        }
                        part1 += num1 * num2;
                    }
                } else {
                    state = State::Nonsense;
                    continue;
                }
            }
        }
    }

    dbg!(part1);
    dbg!(part2);
}

fn is_literal(curr: usize, literal: &[u8], input: &[u8]) -> bool {
    curr + literal.len() <= input.len() && &input[curr..curr + literal.len()] == literal
}
