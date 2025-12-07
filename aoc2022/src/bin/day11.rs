use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Monkey {
    items: Vec<isize>,
    op: Vec<OpToken>,
    test_div: isize,
    true_monkey: usize,
    false_monkey: usize,
    activity: usize,
}

#[derive(Debug, Clone, Copy)]
enum OpToken {
    Old,
    Lit(isize),
    Plus,
    Mult,
}

fn parse(r: impl BufRead) -> Vec<Monkey> {
    let mut monkies = vec![];
    let mut it = r.lines().flatten();
    loop {
        let name = it
            .next()
            .unwrap()
            .strip_prefix("Monkey ")
            .and_then(|s| s.strip_suffix(':'))
            .map(|s| s.parse::<usize>().unwrap())
            .unwrap();
        assert!(name == monkies.len());

        let starting = it
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(',')
            .map(|s| s.trim().parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let op_tokens = it
            .next()
            .unwrap()
            .strip_prefix("  Operation: ")
            .unwrap()
            .split(' ')
            .skip(2)
            .map(|token| match token.trim() {
                "old" => OpToken::Old,
                "+" => OpToken::Plus,
                "*" => OpToken::Mult,
                lit => OpToken::Lit(lit.parse().unwrap()),
            })
            .collect::<Vec<_>>();

        let test: isize = it
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .map(|s| s.parse().unwrap())
            .unwrap();

        let true_monkey: usize = it
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .map(|s| s.parse().unwrap())
            .unwrap();

        let false_monkey: usize = it
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .map(|s| s.parse().unwrap())
            .unwrap();

        monkies.push(Monkey {
            items: starting,
            op: op_tokens,
            test_div: test,
            true_monkey,
            false_monkey,
            activity: 0,
        });

        if it.next().is_none() {
            break;
        }
    }

    monkies
}

fn apply_op(item: isize, tokens: &[OpToken]) -> isize {
    let old = item;
    let v = |t| match t {
        OpToken::Old => old,
        OpToken::Lit(n) => n,
        _ => panic!(),
    };

    assert!(tokens.len() == 3);
    let left = v(tokens[0]);
    let op = tokens[1];
    let right = v(tokens[2]);

    match op {
        OpToken::Plus => left + right,
        OpToken::Mult => left * right,
        _ => panic!(),
    }
}

fn throw_round(monkies: &mut [Monkey], divisor: isize) {
    for i in 0..monkies.len() {
        // for each item a monkey has
        let mut items = vec![];
        std::mem::swap(&mut items, &mut monkies[i].items);

        for item in items {
            let item = apply_op(item, &monkies[i].op); // inspect
            monkies[i].activity += 1; // track activity

            // let item = item / 3; // your worry drop

            // keep worry level down
            let item = item % divisor;

            let test = item % monkies[i].test_div == 0;
            let to = if test {
                monkies[i].true_monkey
            } else {
                monkies[i].false_monkey
            };
            monkies[to].items.push(item);
        }
    }
}

fn main() {
    let mut monkies = parse(BufReader::new(File::open("input.txt").unwrap()));

    let divisor: isize = monkies.iter().map(|m| m.test_div).product();

    for _ in 0..10000 {
        throw_round(&mut monkies, divisor);
    }

    let mut activities: Vec<_> = monkies.iter().map(|m| m.activity).collect();
    activities.sort();
    activities.reverse();
    println!(
        "Monkey business: {}",
        activities.iter().take(2).product::<usize>()
    );
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{parse, throw_round};

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
  
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
  
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
  
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn t() {
        let mut monkies = parse(Cursor::new(INPUT));
        let divisor: isize = monkies.iter().map(|m| m.test_div).product();

        for _ in 0..20 {
            throw_round(&mut monkies, divisor);
        }
        dbg!(monkies);
    }
}
