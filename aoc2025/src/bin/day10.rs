use std::{collections::VecDeque, usize};

use aoc::{StrExt, fetch_input, lines};
use itertools::Itertools;
use rand::{distr, rng, seq::SliceRandom};

#[derive(Debug)]
struct Machine {
    diagram: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_reqs: Vec<isize>,
}

// Insight:
// Order of pressing the buttons does not matter. My tree has nodes for EVERY
// order of pressing the buttons, but really it's just the number. Pressing A
// then B is equivalent to B then A.
//
// N buttons (0, 0, 0, 0, ...). Want any given button can only be pressed a
// maximum number of times max(goal[..])
fn main() {
    let input = lines(fetch_input(2025, 10));
    let input = input
        .iter()
        .map(|s| s.split_whitespace().collect_vec())
        .collect_vec();

    let machines = input
        .iter()
        .map(|m| {
            let [diagram, buttons @ .., jolts] = &m[..] else {
                panic!()
            };
            let diagram = diagram
                .strip_brackets('[', ']')
                .unwrap()
                .chars()
                .map(|c| c == '#')
                .collect_vec();

            let buttons = buttons
                .iter()
                .map(|s| {
                    s.strip_brackets('(', ')')
                        .unwrap()
                        .split_parse::<usize>(",")
                        .collect_vec()
                })
                .collect_vec();

            let jolts = jolts
                .strip_brackets('{', '}')
                .unwrap()
                .split_parse::<isize>(",")
                .collect_vec();

            Machine {
                diagram,
                buttons,
                joltage_reqs: jolts,
            }
        })
        .collect_vec();

    // For part 1, pressing a button twice does nothing. It doesn't matter if
    // you press other buttons in between, it will still toggle a given light
    // twice and therefore have no overall effect. So at maximum we need to
    // press a given button once.

    let part1 = part1(&machines);
    dbg!(part1);
    assert_eq!(447, part1);

    let part2 = part2(&machines);
    dbg!(part2);
}

#[derive(Debug)]
struct Node {
    joltages: Vec<isize>,
    presses: usize,
}

fn part2(machines: &[Machine]) -> usize {
    let mut count = 0;

    // DFS?
    for machine in machines {
        let goal = &machine.joltage_reqs;
        let mut min_presses = usize::MAX;

        let starts = base_solutions_for_req(
            machine,
            &Node {
                joltages: vec![0; machine.joltage_reqs.len()],
                presses: 0,
            },
            2,
        );

        for base in starts {
            let starts = base_solutions_for_req(machine, &base, 7);

            for base in starts {
                let starts = base_solutions_for_req(machine, &base, 0);
                let mut q = VecDeque::from_iter(starts);

                while let Some(n) = q.pop_back() {
                    if n.presses >= min_presses {
                        continue; // already matched this, no point continuing.
                    }

                    if n.joltages.iter().zip(goal).any(|(a, g)| a > g) {
                        // breached our goal.
                        // println!("Breeched");
                        continue;
                    }

                    // println!("Node {}: {:?}", n.presses, n.joltages);

                    // have we acheived the goal?
                    if &n.joltages == goal {
                        println!("Achieved goal {}", n.presses);
                        panic!();
                        min_presses = min_presses.min(n.presses);
                        continue;
                    }

                    // for each possible button press.
                    for button in &machine.buttons[2..] {
                        let mut new = Node {
                            joltages: n.joltages.clone(),
                            presses: n.presses + 1,
                        };
                        for &i in button {
                            new.joltages[i] += 1;
                        }
                        q.push_back(new);
                    }
                }
            }
        }

        count += min_presses;
        println!("Machine done, {} presses", min_presses);
    }

    count
}

// From the given start node, find all starting solutions for the given requirement.
// Idea here is to try and cut down on the depth of the search tree.
fn base_solutions_for_req(machine: &Machine, start: &Node, req_i: usize) -> Vec<Node> {
    let mut nodes = vec![];

    let buttons = machine
        .buttons
        .iter()
        .filter(|b| b.contains(&req_i))
        .collect_vec();

    let overall_goal = machine.joltage_reqs[req_i];
    let presses_needed = overall_goal - start.joltages[req_i];
    if presses_needed < 0 {
        panic!();
    }

    for dist in distribute(presses_needed as usize, buttons.len()) {
        let mut candidate = Node {
            joltages: start.joltages.clone(),
            presses: start.presses + presses_needed as usize,
        };

        for (num, &button) in dist.iter().zip(&buttons) {
            for jolti in button {
                candidate.joltages[*jolti] += *num as isize;
            }
        }

        if candidate
            .joltages
            .iter()
            .zip(&machine.joltage_reqs)
            .any(|(a, g)| *a > *g)
        {
            // breached our goal.
            // println!("Breeched");
            continue;
        }

        println!("Candidate: {:?}", candidate);
        nodes.push(candidate);
        // if n.joltages[req_i] > machine.joltage_reqs[req_i] {
        //     // breached our goal.
        //     // println!("Breeched");
        //     continue;
        // }

        // // have we acheived the goal?
        // if n.joltages[req_i] == machine.joltage_reqs[req_i] {
        //     // println!("Achieved goal");
        //     sub_sols += 1;
        //     if sub_sols % 10000 == 0 {
        //         println!("{sub_sols} solutons... {}, {:?}", n.presses, n.joltages);
        //     }
        //     continue;
        // }

        // // for each possible button press.
        // for &button in &buttons {
        //     let mut new = Node {
        //         joltages: n.joltages.clone(),
        //         presses: n.presses + 1,
        //     };
        //     for i in button {
        //         new.joltages[*i] += 1;
        //     }
        //     q.push_back(new);
        // }
    }

    nodes
}

fn part1(machines: &[Machine]) -> usize {
    let mut count = 0usize;

    for machine in machines {
        // beyond last bit pattern to test.
        let bits_end = 1usize << machine.buttons.len();
        let mut min_presses = u32::MAX;

        for bits in 0..bits_end {
            let mut lights = 0;
            for buttoni in 0..machine.buttons.len() {
                // press the button?
                let press = 0 != (bits & (1usize << buttoni));
                if press {
                    lights ^= buttons_bit_pattern(&machine.buttons[buttoni]);
                }
            }

            if lights == diagram_bit_pattern(&machine.diagram) {
                let presses = bits.count_ones();
                min_presses = min_presses.min(presses);
            }
        }

        count += min_presses as usize;
    }

    count
}

fn diagram_bit_pattern(diagram: &[bool]) -> usize {
    let mut pat = 0;
    for i in 0..diagram.len() {
        pat |= (diagram[i] as usize) << i;
    }

    pat
}

fn buttons_bit_pattern(buttons: &[usize]) -> usize {
    let mut pat = 0;
    for b in buttons {
        pat |= 1 << b;
    }
    pat
}

/// Oh my god.
fn distribute(n: usize, over: usize) -> Vec<Vec<usize>> {
    if over == 1 {
        vec![vec![n]]
    } else {
        let mut vs = vec![];
        for i in 0..n {
            for end in distribute(n - i, over - 1) {
                let mut v = vec![i];
                v.extend_from_slice(&end);
                vs.push(v);
            }
        }

        vs
    }
}

#[cfg(test)]
mod test {
    use crate::distribute;

    #[test]
    fn test_divider() {
        let divs = distribute(100, 5);
        println!("{:?}", divs[2421275]);
    }
}
