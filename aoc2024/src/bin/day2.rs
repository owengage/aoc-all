use aoc::{fetch_input, lines};

fn main() {
    let input = lines(fetch_input(2024, 2));
    let reports: Vec<_> = input
        .into_iter()
        .map(|line| {
            let vals: Vec<usize> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            vals
        })
        .collect();

    let part1: usize = reports
        .iter()
        .map(|report| fully_safe(report) as usize)
        .sum();

    dbg!(part1);

    let part2: usize = reports
        .iter()
        .map(|report| {
            if fully_safe(report) {
                return 1;
            }
            for i in 0..report.len() {
                let mut sub = report.clone();
                sub.remove(i);

                if fully_safe(&sub) {
                    return 1;
                }
            }
            0
        })
        .sum();

    dbg!(part2);
}

fn fully_safe(report: &[usize]) -> bool {
    let mut up = false;
    let mut down = false;
    let mut diffsafe = true;
    report.windows(2).for_each(|w| {
        if w[0] < w[1] {
            up = true;
        }
        if w[0] > w[1] {
            down = true;
        }
        let diff = w[0].abs_diff(w[1]);
        if diff > 3 || diff == 0 {
            diffsafe = false;
        }
    });

    up ^ down && diffsafe
}
