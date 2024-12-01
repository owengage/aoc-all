use core::panic;
use std::collections::HashMap;

use aoc::line_blocks;
use aoc::StrExt;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    Op {
        descriptor: char,
        op: char,
        rhs: usize,
        dest: String,
    },
    Term {
        dest: String,
    },
}

fn main() {
    let input = line_blocks("aoc2023/input/work19");
    let workflows = parse_workflows(&input[0]);
    let parts = parse_parts(&input[1]);

    let part1 = part1(&workflows, &parts);
    dbg!(part1);
}

fn part1(workflows: &HashMap<String, Vec<Rule>>, parts: &[Part]) -> usize {
    let mut count = 0;

    for part in parts {
        let mut wf = &workflows["in"];
        loop {
            let next = eval(part, wf);
            if next == "A" {
                count += part.x + part.m + part.a + part.s;
                break;
            }
            if next == "R" {
                break;
            }

            wf = &workflows[next];
        }
    }

    count
}

fn eval<'r>(part: &Part, wf: &'r [Rule]) -> &'r str {
    for rule in wf {
        match rule {
            Rule::Op {
                descriptor,
                op,
                rhs,
                dest,
            } => {
                let lhs = match descriptor {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!(),
                };
                let result = match op {
                    '<' => lhs < *rhs,
                    '>' => lhs > *rhs,
                    _ => panic!(),
                };
                if result {
                    return dest;
                }
            }
            Rule::Term { dest } => return dest,
        }
    }
    unreachable!()
}

fn parse_parts(input: &[String]) -> Vec<Part> {
    let mut parts = vec![];

    for line in input {
        let subparts = line.as_str().strip_brackets('{', '}').unwrap().split(',');
        let subparts: HashMap<char, usize> = subparts
            .map(|r| {
                let (letter, value) = r.split_once('=').unwrap();
                (letter.chars().next().unwrap(), value.parse().unwrap())
            })
            .collect();

        parts.push(Part {
            x: subparts[&'x'],
            m: subparts[&'m'],
            a: subparts[&'a'],
            s: subparts[&'s'],
        })
    }

    parts
}

fn parse_workflows(input: &[String]) -> HashMap<String, Vec<Rule>> {
    let mut workflows = HashMap::new();

    for line in input {
        let (name, rest) = line.split_once('{').unwrap();
        let rules = rest
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(parse_rule)
            .collect_vec();

        workflows.insert(name.to_owned(), rules);
    }

    workflows
}

fn parse_rule(r: &str) -> Rule {
    // eg: a<2006:qkq
    let Some((op, dest)) = r.split_once(':') else {
        return Rule::Term {
            dest: r.to_string(),
        };
    };

    let rhs = op[2..].parse().unwrap();
    let descriptor = op[0..1].chars().next().unwrap();
    let op = op[1..2].chars().next().unwrap();

    Rule::Op {
        descriptor,
        op,
        rhs,
        dest: dest.to_string(),
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_parse() {}
}
