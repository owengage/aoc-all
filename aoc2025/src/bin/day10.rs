use std::collections::HashMap;

use aoc::{StrExt, fetch_input, lines};
use itertools::Itertools;

#[derive(Debug)]
struct Machine {
    diagram: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_reqs: Vec<isize>,
}

// Try parity based solution: https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/

fn main() {
    let input = lines(fetch_input(2025, 10));
    let machines = input.iter().map(|s| parse_machine(&s)).collect_vec();
    let part1 = part1(&machines);
    dbg!(part1);
    assert_eq!(447, part1);

    let part2 = part2(&machines);
    dbg!(part2);
    assert_eq!(18960, part2);
}

fn parse_machine(line: &str) -> Machine {
    let line = line.split_whitespace().collect_vec();

    let [diagram, buttons @ .., jolts] = &line[..] else {
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
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    joltages_left: Vec<isize>,
    buttons: Vec<Vec<usize>>,
    presses: isize,
}

fn part2(machines: &[Machine]) -> isize {
    let mut total_presses = 0;

    for machine in machines {
        let sol = solve_machine(machine);
        assert_ne!(isize::MAX, sol);
        total_presses += sol;
    }

    total_presses
}

fn solve_machine(machine: &Machine) -> isize {
    let node = Node {
        joltages_left: machine.joltage_reqs.clone(),
        buttons: machine.buttons.clone(),
        presses: 0,
    };

    parity_solve(&node)
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

fn parity_solve(machine: &Node) -> isize {
    let ns = solve_for_even_parity(machine);
    let mut min_presses = isize::MAX;
    let mut cache = HashMap::new();

    if machine.joltages_left.iter().all(|j| *j == 0) {
        return 0;
    }

    for (presses, mut n) in ns {
        // Half the now even joltage requirements.
        n.joltages_left.iter_mut().for_each(|j| *j /= 2);

        let subsolve = if cache.contains_key(&n) {
            *cache.get(&n).unwrap()
        } else {
            let sol = parity_solve(&n);
            cache.insert(n, sol);
            sol
        };

        if subsolve == isize::MAX {
            continue;
        }

        let n_min = presses + subsolve * 2;
        min_presses = min_presses.min(n_min);
    }

    min_presses
}

// Return all solutions that convert to even parity.
fn solve_for_even_parity(machine: &Node) -> Vec<(isize, Node)> {
    let bits_end = 1usize << machine.buttons.len();
    let mut nodes = vec![];

    for bits in 0..bits_end {
        let mut lights = 0;
        let mut new_joltage = machine.joltages_left.clone();

        for buttoni in 0..machine.buttons.len() {
            // press the button?
            let press = 0 != (bits & (1usize << buttoni));
            if press {
                lights ^= buttons_bit_pattern(&machine.buttons[buttoni]);
                for &i in &machine.buttons[buttoni] {
                    new_joltage[i] -= 1;
                }
            }
        }

        if new_joltage.iter().any(|j| *j < 0) {
            continue;
        }

        let diagram = machine
            .joltages_left
            .iter()
            .map(|j| (*j % 2) != 0)
            .collect_vec();

        if lights == diagram_bit_pattern(&diagram) {
            let presses = bits.count_ones() as isize;
            nodes.push((
                presses,
                Node {
                    joltages_left: new_joltage,
                    buttons: machine.buttons.clone(),
                    presses: 0,
                },
            ));
        }
    }

    nodes
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

#[cfg(test)]
mod test {
    use crate::{Machine, Node, parity_solve, parse_machine, solve_for_even_parity, solve_machine};

    #[test]
    fn parity() {
        let m = parse_machine("[....] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        let n = Node {
            joltages_left: m.joltage_reqs,
            buttons: m.buttons,
            presses: 0,
        };

        println!("{:#?}", solve_for_even_parity(&n));

        dbg!(parity_solve(&n));
    }

    #[test]
    fn solve_m() {
        let m = Machine {
            diagram: vec![],
            buttons: vec![vec![0]],
            joltage_reqs: vec![100],
        };

        assert_eq!(100, solve_machine(&m));
    }

    #[test]
    fn solve_m2() {
        let m = Machine {
            diagram: vec![],
            buttons: vec![vec![0]],
            joltage_reqs: vec![100, 1],
        };

        assert_eq!(isize::MAX, solve_machine(&m));
    }

    #[test]
    fn solve_m3() {
        assert_eq!(
            101,
            solve_machine(&Machine {
                diagram: vec![],
                buttons: vec![vec![0], vec![1]],
                joltage_reqs: vec![100, 1],
            })
        );

        assert_eq!(
            100,
            solve_machine(&Machine {
                diagram: vec![],
                buttons: vec![vec![0, 1], vec![1]],
                joltage_reqs: vec![100, 100],
            })
        );

        assert_eq!(
            100,
            solve_machine(&Machine {
                diagram: vec![],
                buttons: vec![vec![1], vec![0, 1]],
                joltage_reqs: vec![100, 100],
            })
        );

        // [...#] (0,3) (0,1,2) (1) (1,3) (1,2) {23,176,26,19}
        assert_eq!(
            176,
            solve_machine(&Machine {
                diagram: vec![],
                buttons: vec![vec![0, 3], vec![0, 1, 2], vec![1], vec![1, 3], vec![1, 2],],
                joltage_reqs: vec![23, 176, 26, 19],
            })
        );

        assert_eq!(
            12,
            solve_machine(&parse_machine(
                "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"
            ))
        );
        assert_eq!(
            11,
            solve_machine(&parse_machine(
                "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            ))
        );

        // from input
        assert_eq!(
            170, // maybe?
            solve_machine(&parse_machine(
                "[####.] (0,3,4) (1,2) (1,2,4) {148,22,22,148,160}"
            ))
        );
    }
}
