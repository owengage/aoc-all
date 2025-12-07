use core::panic;
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
enum Data {
    Int(i64),
    List(Vec<Data>),
}

#[derive(Debug, Deserialize)]
struct Pair {
    left: Vec<Data>,
    right: Vec<Data>,
}

fn is_in_order(left: &[Data], right: &[Data]) -> Ordering {
    let max_len = left.len().max(right.len());

    if max_len == 0 {
        return Ordering::Equal;
    }

    for i in 0..max_len {
        let left = left.get(i);
        let right = right.get(i);

        match (left, right) {
            (None, None) => panic!("how to order?"),
            (None, Some(_)) => {
                // left side ran out, so we're in order...
                return Ordering::Less;
            }
            (Some(_), None) => {
                // right side ran out, not in right order
                return Ordering::Greater;
            }
            (Some(Data::Int(left)), Some(Data::Int(right))) => {
                if left == right {
                    continue;
                }
                return if left < right {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }
            (Some(Data::Int(left)), Some(Data::List(right))) => {
                let left = vec![Data::Int(*left)];
                match is_in_order(&left, right) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    _ => {}
                }
            }
            (Some(Data::List(left)), Some(Data::Int(right))) => {
                let right = vec![Data::Int(*right)];
                match is_in_order(left, &right) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    _ => {}
                }
            }
            (Some(Data::List(left)), Some(Data::List(right))) => match is_in_order(left, right) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                _ => {}
            },
        }
    }

    Ordering::Equal
}

fn main() {
    let pairs = parse(BufReader::new(File::open("input.txt").unwrap()));

    let a = 1.2;
    let b = f32::NAN;

    if a > b {
        todo!()
    }

    let part1 = pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| is_in_order(&pair.left, &pair.right) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let mut packets: Vec<Vec<Data>> = pairs
        .into_iter()
        .flat_map(|pair| [pair.left, pair.right])
        .collect();

    // Divider packets.
    let div1 = vec![Data::List(vec![Data::Int(2)])];
    let div2 = vec![Data::List(vec![Data::Int(6)])];
    packets.push(div1.clone());
    packets.push(div2.clone());

    packets.sort_by(|a, b| is_in_order(a, b));

    println!("{:#?}", packets);

    let mut i1 = 0;
    let mut i2 = 0;

    for (i, pkt) in packets.iter().enumerate() {
        if *pkt == div1 {
            i1 = i + 1;
        }
        if *pkt == div2 {
            i2 = i + 1;
        }
    }

    dbg!(i1 * i2);

    // guessed 74, wrong.
    dbg!(part1);
}

fn parse(r: impl BufRead) -> Vec<Pair> {
    let lines: Vec<_> = r.lines().flatten().filter(|l| !l.is_empty()).collect();
    lines
        .chunks_exact(2)
        .map(|pair| {
            let left = serde_json::from_str(&pair[0]).unwrap();
            let right = serde_json::from_str(&pair[1]).unwrap();
            Pair { left, right }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use crate::{is_in_order, Data};

    // #[test]
    // fn simple() {
    //     assert!(is_in_order(&[Data::Int(1)], &[Data::Int(2)]).unwrap());
    //     assert!(is_in_order(&[Data::Int(1)], &[Data::Int(1)]).is_none());
    // }
    // #[test]
    // fn case1() {
    //     let left = [1, 1, 3, 1, 1].map(Data::Int);
    //     let right = [1, 1, 5, 1, 1].map(Data::Int);

    //     assert!(is_in_order(&left, &right).unwrap());
    // }

    // #[test]
    // fn case2() {
    //     let left = serde_json::from_str::<Vec<Data>>("[[1],[2,3,4]]").unwrap();
    //     let right = serde_json::from_str::<Vec<Data>>("[[1],4]").unwrap();
    //     assert!(is_in_order(&left, &right).unwrap());
    // }

    // #[test]
    // fn case3() {
    //     let left = serde_json::from_str::<Vec<Data>>("[9]").unwrap();
    //     let right = serde_json::from_str::<Vec<Data>>("[[8,7,6]]").unwrap();
    //     assert!(!is_in_order(&left, &right).unwrap());
    // }

    // #[test]
    // fn case4() {
    //     let left = serde_json::from_str::<Vec<Data>>("[[4,4],4,4]").unwrap();
    //     let right = serde_json::from_str::<Vec<Data>>("[[4,4],4,4,4]").unwrap();
    //     assert!(is_in_order(&left, &right).unwrap());
    // }

    // #[test]
    // fn case5() {
    //     let left = serde_json::from_str::<Vec<Data>>("[7,7,7,7]").unwrap();
    //     let right = serde_json::from_str::<Vec<Data>>("[7,7,7]").unwrap();
    //     assert!(!is_in_order(&left, &right).unwrap());
    // }

    // #[test]
    // fn case6() {
    //     let left = serde_json::from_str::<Vec<Data>>("[]").unwrap();
    //     let right = serde_json::from_str::<Vec<Data>>("[3]").unwrap();
    //     assert!(is_in_order(&left, &right).unwrap());
    // }

    // #[test]
    // fn case7() {
    //     let left = serde_json::from_str::<Vec<Data>>("[[[]]]").unwrap();
    //     let right = serde_json::from_str::<Vec<Data>>("[[]]").unwrap();
    //     assert!(!is_in_order(&left, &right).unwrap());
    // }

    #[test]
    fn case8() {
        let left = serde_json::from_str::<Vec<Data>>("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        let right = serde_json::from_str::<Vec<Data>>("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();
        assert_eq!(is_in_order(&left, &right), Ordering::Greater);
    }
}
