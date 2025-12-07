use core::panic;
use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use aoc::{fetch_input, text};

#[derive(Debug)]
struct Snafu(isize);

fn main() {
    // 2 == 2
    // 1 == 1
    // 0 == 0
    // - == -1
    // = == -2
    // 615, 125, 25, 1s
    let numbers: Vec<_> = parse(&text(fetch_input(2022, 25)));

    let total = numbers.into_iter().map(|s| s.0).sum::<isize>();

    println!("{}", Snafu(total));

    // println!("{:?}", numbers);
}

impl FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snafu(
            s.chars()
                .rev()
                .enumerate()
                .map(|(power, digit)| {
                    let factor = 5isize.pow(power as u32);
                    let d = match digit {
                        '2' => 2,
                        '1' => 1,
                        '0' => 0,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!(),
                    };

                    d * factor
                })
                .sum::<isize>(),
        ))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut digits = vec![];
        let signum = self.0.signum();
        let mut remains = self.0.abs();
        let mut p = 0;

        while 5isize.pow(p + 1) / 2 < remains {
            p += 1;
        }

        // What multiple of this power puts is in range?
        for p in (0..=p).rev() {
            let unit = 5isize.pow(p);
            let delta = unit / 2;

            for i in -2..=2 {
                let range = i * unit - delta..=i * unit + delta;
                if range.contains(&remains) {
                    digits.push(i);
                    remains -= i * unit;
                    break;
                }
            }
        }

        for digit in digits {
            f.write_char(match digit * signum {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => panic!(),
            })?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> Vec<Snafu> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}
