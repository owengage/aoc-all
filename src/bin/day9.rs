use aoc::lines;

fn main() {
    let input = lines("input/work9");
    let histories = parse_histories(&input);

    let part1: isize = histories.iter().map(|h| predict_next(h.clone())).sum();
    let part2: isize = histories
        .into_iter()
        .map(|mut h| {
            h.reverse();
            predict_next(h)
        })
        .sum();

    dbg!(part1);
    dbg!(part2);
}

fn predict_next(hist: Vec<isize>) -> isize {
    let mut levels = vec![hist];

    loop {
        let last = levels.last().unwrap();
        let next = last.windows(2).map(|win| win[1] - win[0]).collect();
        levels.push(next);

        if levels.last().unwrap().iter().all(|n| *n == 0) {
            break;
        }
    }

    // now bubble up
    let mut next_val = 0;

    for level in levels.iter().rev() {
        let last_at_level = level.last().unwrap();
        next_val += last_at_level;
    }

    next_val
}

fn parse_histories(input: &[String]) -> Vec<Vec<isize>> {
    input
        .iter()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::lines;

    #[test]
    fn test_hist() {
        let input = lines("input/day9");
        let histories = parse_histories(&input);
        let part1: isize = histories.iter().map(|h| predict_next(h.clone())).sum();
        let ans = 1992273652;
        assert_eq!(part1, ans);
    }
}
