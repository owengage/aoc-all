use std::collections::HashMap;

use aoc::{fetch_input, lines};
use itertools::Itertools;

fn main() {
    let input = lines(fetch_input(2025, 11));
    let adj = input
        .iter()
        .flat_map(|s| {
            let (src, dsts) = s.split_once(':').unwrap();
            dsts.trim().split(' ').map(|dst| (src, dst)).collect_vec()
        })
        .sorted()
        .collect_vec();

    let part1 = paths(&mut HashMap::new(), &adj, "you", "out");
    dbg!(part1);
    assert_eq!(634, part1);

    let part2 = part2(&adj);
    dbg!(part2);
    assert_eq!(377452269415704, part2);
}

fn part2(adj: &[(&str, &str)]) -> usize {
    let mut cache = HashMap::new();
    let svr_to_dac = paths(&mut cache, adj, "svr", "dac");
    let svr_to_fft = paths(&mut cache, adj, "svr", "fft");
    let dac_to_fft = paths(&mut cache, adj, "dac", "fft");
    let fft_to_dac = paths(&mut cache, adj, "fft", "dac");
    let dac_to_out = paths(&mut cache, adj, "dac", "out");
    let fft_to_out = paths(&mut cache, adj, "fft", "out");

    // one of these will be zero since this is a DAG.
    let p1 = svr_to_dac * dac_to_fft * fft_to_out;
    let p2 = svr_to_fft * fft_to_dac * dac_to_out;
    p1 + p2
}

fn paths<'i>(
    cache: &mut HashMap<(&'i str, &'i str), usize>,
    adj: &[(&'i str, &'i str)],
    from: &'i str,
    to: &'i str,
) -> usize {
    if let Some(c) = cache.get(&(from, to)) {
        return *c;
    }

    let connectors = adj.iter().filter(|(_, t)| *t == to).map(|(a, _)| a);
    let mut sum = 0;

    for &connector in connectors {
        if connector == from {
            sum += 1; // base case
        } else {
            sum += paths(&mut *cache, adj, from, connector);
        }
    }

    cache.insert((from, to), sum);
    sum
}
