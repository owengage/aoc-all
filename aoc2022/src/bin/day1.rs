use aoc::{fetch_input, lines};

fn main() {
    let input = lines(fetch_input(2022, 1));

    let mut res = input
        .into_iter()
        .fold((vec![], 0), |(mut all, current), next| {
            if next.is_empty() {
                all.push(current);
                (all, 0)
            } else {
                (all, current + next.parse::<i32>().unwrap())
            }
        })
        .0;

    res.sort();

    println!("part 1: {}", res.last().unwrap());
    println!("part 2: {}", res.iter().rev().take(3).sum::<i32>());
}
