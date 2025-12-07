use std::collections::{HashMap, VecDeque};

use aoc::{fetch_input, text};
use num::Rational64;
use parse::RawType;

mod parse {
    pub struct RawNode<'input> {
        pub name: &'input str,
        pub data: RawType<'input>,
    }

    pub enum RawType<'input> {
        Op {
            op: char,
            a: &'input str,
            b: &'input str,
        },
        Literal(isize),
    }

    pub fn parse(input: &str) -> Vec<RawNode<'_>> {
        input
            .lines()
            .map(|s| {
                let (name, data) = s.split_once(": ").unwrap();
                if let Ok(lit) = data.parse::<isize>() {
                    RawNode {
                        name,
                        data: RawType::Literal(lit),
                    }
                } else {
                    let mut data = data.split_whitespace();
                    let a = data.next().unwrap();
                    let op = data.next().unwrap();
                    let b = data.next().unwrap();
                    RawNode {
                        name,
                        data: RawType::Op {
                            op: op.chars().next().unwrap(),
                            a,
                            b,
                        },
                    }
                }
            })
            .collect()
    }
}

fn main() {
    let binding = text(fetch_input(2022, 21));
    let monkies = parse::parse(&binding);
    let (lookup, mut graph) = monkey_passports(monkies);
    let _inverted_lookup: HashMap<_, _> = lookup.iter().map(|(&a, &b)| (b, a)).collect();

    let root_idx = lookup["root"];

    println!("part1: {}", graph[root_idx].eval(&graph));

    let root = match graph[root_idx] {
        Node::Op {
            op: _,
            a,
            b,
            parent,
        } => Node::Op {
            parent,
            op: '=',
            a,
            b,
        },
        Node::Literal { .. } => panic!("should be op"),
    };
    graph[root_idx] = root;

    let humn_idx = lookup["humn"];

    // We want to detach the two subtrees under root, get the value of the other
    // side so we know what we're aiming for with the humn side.

    let humn_subtree_root = find_roots_child(humn_idx, &graph).unwrap();
    let other_root = match graph[root_idx] {
        Node::Op { a, .. } => match humn_subtree_root {
            // Note: imported this into aoc-all repo and this code clearly looks
            // broken. Replacing it to just remove warnings.
            // a => b,
            // b => a,
            // _ => panic!(),
            _ => a,
        },
        Node::Literal {
            parent: _parent,
            value: _value,
        } => panic!(),
    };

    // Separate the trees.
    graph[humn_subtree_root].orphan();
    graph[other_root].orphan();

    let target = graph[other_root].eval(&graph);

    let path = path_to(humn_idx, &graph);
    let var = part2(&path, target, &graph);

    graph[humn_idx] = Node::Literal {
        parent: None,
        value: var,
    };
    let ours = graph[humn_subtree_root].eval(&graph);

    println!("humn: \t{}", var);
    println!("Ours: \t{}", ours.numer() / ours.denom());
    println!("Target: \t{}", target);
}

fn part2(path: &[usize], target: Rational64, graph: &[Node]) -> Rational64 {
    let mut current = target;

    for win in path.windows(2) {
        let (current_idx, next_idx) = (win[0], win[1]);

        let node = &graph[current_idx];
        match node {
            Node::Op { op, a, b, .. } => {
                if next_idx == *a {
                    // we are a-side.
                    let bval = graph[*b].eval(graph);
                    match &op {
                        '/' => current *= bval,
                        '*' => current /= bval,
                        '-' => current += bval,
                        '+' => current -= bval,
                        _ => panic!(),
                    };
                } else {
                    // we are b-side
                    let aval = graph[*a].eval(graph);
                    match &op {
                        '/' => current = aval / current,
                        '*' => current /= aval,
                        '-' => current = aval - current,
                        '+' => current -= aval,
                        _ => panic!(),
                    };
                }
            }
            Node::Literal { .. } => panic!(),
        }
    }

    current
}

#[derive(Debug, Clone)]
enum Node {
    Op {
        parent: Option<usize>,
        op: char,
        a: usize,
        b: usize,
    },
    Literal {
        parent: Option<usize>,
        value: num::Rational64,
    },
}

impl Node {
    fn eval(&self, graph: &[Node]) -> num::Rational64 {
        match self {
            Node::Op { op, a, b, .. } => {
                let a = graph[*a].eval(graph);
                let b = graph[*b].eval(graph);
                match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => {
                        // assert!(a % b == 0f64, "a: {}, b: {}", a, b);
                        a / b
                    }
                    '=' => Rational64::new((a == b) as i64, 1),
                    _ => panic!("unknown op"),
                }
            }
            Node::Literal { value, .. } => *value,
        }
    }

    fn i_am_your_father(&mut self, parent_idx: usize) {
        match self {
            Node::Op { parent, .. } => *parent = Some(parent_idx),
            Node::Literal { parent, .. } => *parent = Some(parent_idx),
        }
    }

    fn parent(&self) -> Option<usize> {
        match self {
            Node::Op { parent, .. } => *parent,
            Node::Literal { parent, .. } => *parent,
        }
    }

    fn orphan(&mut self) {
        match self {
            Node::Op { parent, .. } => *parent = None,
            Node::Literal { parent, .. } => *parent = None,
        }
    }
}

/// Find the root node from start, returning the index of the child node of the
/// root that start is within.
fn find_roots_child(start: usize, graph: &[Node]) -> Option<usize> {
    let mut previous = None;
    let mut idx = start;

    loop {
        let node = &graph[idx];
        match node.parent() {
            Some(parent) => {
                previous = Some(idx);
                idx = parent;
            }
            None => break,
        }
    }

    previous
}

fn path_to(start: usize, graph: &[Node]) -> Vec<usize> {
    let mut previous = vec![start];
    let mut idx = start;

    loop {
        let node = &graph[idx];
        match node.parent() {
            Some(parent) => {
                previous.push(parent);
                idx = parent;
            }
            None => break,
        }
    }

    previous.reverse();
    previous
}

fn monkey_passports(monkies: Vec<parse::RawNode<'_>>) -> (HashMap<&str, usize>, Vec<Node>) {
    let mut lookup = HashMap::new();
    let mut graph = Vec::with_capacity(monkies.len());
    let mut names = Vec::with_capacity(monkies.len());

    for m in &monkies {
        lookup.insert(m.name, names.len());
        names.push(m.name);
    }

    for m in monkies {
        graph.push(match m.data {
            RawType::Op { op, a, b } => Node::Op {
                parent: None,
                op,
                a: lookup[a],
                b: lookup[b],
            },
            RawType::Literal(v) => Node::Literal {
                parent: None,
                value: Rational64::new(v as i64, 1),
            },
        })
    }

    // Set up parents.
    let root_idx = lookup["root"];
    let mut q = VecDeque::<usize>::new();
    q.push_back(root_idx);

    while let Some(parent_idx) = q.pop_back() {
        let node = graph[parent_idx].clone();
        match node {
            Node::Op { a, b, .. } => {
                graph[a].i_am_your_father(parent_idx);
                graph[b].i_am_your_father(parent_idx);
                q.push_back(a);
                q.push_back(b);
            }
            Node::Literal { .. } => {}
        }
    }

    (lookup, graph)
}
