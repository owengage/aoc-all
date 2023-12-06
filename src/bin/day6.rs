use aoc::lines;

fn main() {
    let input = lines("input/day6");
    let races = parse_races(input.clone());
    let race_part2 = parse_race_part2(input);

    let part1: usize = races
        .iter()
        .map(|r| {
            let hs = hold_times(*r);
            hs[1] - hs[0] + 1 // 1-3 is three values not 3-1=2, so plus 1.
        })
        .product();

    let part2 = hold_times(race_part2);
    let part2 = part2[1] - part2[0] + 1;
    dbg!(part1);
    dbg!(part2);
}

fn parse_race_part2(input: Vec<String>) -> Race {
    let f = |l: &str| -> usize {
        l.split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap()
    };

    Race {
        time: f(&input[0]),
        record_distance: f(&input[1]),
    }
}

fn hold_times(race: Race) -> [usize; 2] {
    let d = race.record_distance;
    let t = race.time;

    // Want point where distance > record_distance
    // where does h * (t-h) == d. Solve for h.
    // ht - hh = d
    // ht - hh - d = 0
    // -h^2 + th - d = 0
    // This is a quadratic.
    let hs = quads(-1.0, t as f64, -(d as f64));

    [hs[0].ceil() as usize, hs[1].floor() as usize]
}

fn quads(a: f64, b: f64, c: f64) -> [f64; 2] {
    let root = (b * b - 4.0 * a * c).sqrt();
    assert!(!root.is_nan());
    let x1 = (-b - root) / (2.0 * a);
    let x2 = (-b + root) / (2.0 * a);

    let mut res = [x1, x2];
    // Feel like the sort might be unnecessary?
    res.sort_by(|a, b| a.partial_cmp(b).unwrap());
    res
}

#[derive(Debug, Clone, Copy)]
struct Race {
    time: usize,
    record_distance: usize,
}

fn parse_races(input: Vec<String>) -> Vec<Race> {
    let f = |l: &str| {
        l.split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect()
    };

    let times = f(&input[0]);
    let distances: Vec<_> = f(&input[1]);

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            time,
            record_distance: distance,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{hold_times, Race};

    #[test]
    fn test_min() {
        let r = Race {
            record_distance: 5,
            time: 5,
        };
        assert_eq!(2, hold_times(r)[0]);
    }
}
