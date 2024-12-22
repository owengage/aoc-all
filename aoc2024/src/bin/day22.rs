use std::collections::HashMap;

use aoc::{fetch_input, lines};
use itertools::Itertools;

fn main() {
    let secrets: Vec<usize> = lines(fetch_input(2024, 22))
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let part1: usize = secrets
        .iter()
        .map(|start| {
            let mut secret = *start;
            for _ in 0..2000 {
                secret = evolve(secret);
            }
            secret
        })
        .sum();

    // There's 19 possible values of the change, so 19^4 ~= 130,000.
    // 2500 buyers for which we'll need to generate 2000 numbers.

    let mut monkeys = vec![];

    for secret in &secrets {
        let mut tracking = HashMap::<[isize; 4], usize>::new();

        let all_secrets = buyer_secrets(*secret);

        for w in all_secrets.windows(5) {
            let deltas = [
                ones(w[1]) - ones(w[0]),
                ones(w[2]) - ones(w[1]),
                ones(w[3]) - ones(w[2]),
                ones(w[4]) - ones(w[3]),
            ];

            let bananas = ones(w[4]);

            let _ = *tracking.entry(deltas).or_insert(bananas as usize);
        }

        monkeys.push(tracking);
    }

    let mut max_bananas = 0;

    // For each delta...
    for delta in (0..4).map(|_| (-9..=9)).multi_cartesian_product() {
        let mut bananas = 0;
        for monkey in &monkeys {
            bananas += monkey.get(&delta[..4]).cloned().unwrap_or_default();
        }
        max_bananas = max_bananas.max(bananas);
    }

    println!("part1 = {part1}");
    println!("part2 = {}", max_bananas);
}

fn ones(step: usize) -> isize {
    (step % 10) as isize
}

fn buyer_secrets(mut secret: usize) -> Vec<usize> {
    let mut secrets = vec![secret];
    for _ in 0..2000 {
        secret = evolve(secret);
        secrets.push(secret);
    }

    secrets
}

fn evolve(mut secret: usize) -> usize {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2048));

    secret
}

fn mix(secret: usize, value: usize) -> usize {
    secret ^ value
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

#[cfg(test)]
mod test {
    use crate::evolve;

    #[test]
    fn test_parse() {
        let mut secret = 123;
        let it = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        for next in it {
            secret = evolve(secret);
            assert_eq!(secret, next);
        }
    }
}
