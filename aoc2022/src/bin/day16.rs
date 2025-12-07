use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::Hash,
};

use std::fmt::Debug;

use regex::Regex;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Label([char; 2]);

impl Label {
    fn new(s: &str) -> Label {
        let s = s.as_bytes();
        Self([s[0] as char, s[1] as char])
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0[0])?;
        f.write_char(self.0[1])
    }
}

struct Input {
    flows: HashMap<Label, isize>,
    adj: Vec<(Label, Label)>,
}

#[derive(Debug)]
struct Node {
    me: Inner,
    ele: Inner,
    children: Vec<Node>, // maybe first_child and next_sibling instead?
}

#[derive(Debug, Clone)]
struct Inner {
    label: Label,

    /// How much time left from this node.
    time_remaining: isize,

    /// Sum total of the value up to this point, including self.
    total_value: isize,
}

fn main() {
    // Plan
    //
    // Every minute is a decision to stay, open a valve, or move to an available
    // node. I'm going to convert this graph into data where for a given node I
    // have the fastest time to travel to any other node in the graph.
    //
    // I will assume that any move to a node in this form (which may consisitute
    // several minutes) is to open a valve. This means I can represent this
    // problem as a tree-searching problem.
    //
    // Naively this tree is going to get intractably big. Part 2 is meant to
    // make it worse somehow, maybe by extending from 30 minutes to something
    // else. This means I need a way to prune the tree.
    //
    // I can do this by calculating a theoretical maximum a given choice could
    // produce. If I travel to a node, I can calculate the best possible return
    // for that node by assuming that I could travel to the best next valve in a
    // minute, and open it, then the next best in the next minute.
    //
    // If the *real* value of any given node adds up to greater than this
    // amount, I can prune that entire theoretical tree. My hopes are that this
    // brings down the search space to a tractablle amount.
    //
    // Each node of this tree looks like it's going to have to track a fair
    // amount of information. I'm not sure on the best way to do that.
    let Input { flows, adj } = parse(&std::fs::read_to_string("input.txt").unwrap());

    // Got our adjencency list, now want to know how many minutes it takes to
    // get from any given node to another. This is a shortest path alg run on
    // each point.
    let paths = shortest_paths(&adj);

    // We can filter out any paths to flow=0 nodes, we'll never want to waste
    // time opening those valves.
    let paths: HashMap<_, _> = paths
        .into_iter()
        .filter(|&((_from, to), _cost)| flows[&to] != 0)
        .collect();

    // Now we want to build a tree representing the entire search space. It's
    // too big to actually store. Let's go one level to start with.

    let time = 26;

    let mut root = Node {
        me: Inner {
            label: Label::new("AA"),
            time_remaining: time,
            total_value: 0,
        },
        ele: Inner {
            label: Label::new("AA"),
            time_remaining: time,
            total_value: 0,
        },
        children: vec![],
    };

    let mut max_seen_flow = 1986;
    expand_all(&mut root, paths, &flows, &mut max_seen_flow);

    let part1 = max_seen_flow;

    // Okay, so how do we build this tree, remembering it's going to have to be
    // mutable... Rc?

    dbg!(part1);
}

fn expand_all(
    root: &mut Node,
    paths: HashMap<(Label, Label), isize>,
    flows: &HashMap<Label, isize>,
    max_seen_flow: &mut isize,
) {
    // We filter out any paths *to* the root node so that we don't
    // revisit in in future, we've already opened the valve. This doesn't
    // prevent us from 'passing through it'. We don't model that.
    let paths: HashMap<(Label, Label), isize> = paths
        .into_iter()
        .filter_map(|((from, to), cost)| {
            if to == root.me.label || to == root.ele.label {
                None
            } else {
                Some(((from, to), cost))
            }
        })
        .collect();

    expand(root, paths.clone(), flows, max_seen_flow);

    for child in &mut root.children {
        expand_all(child, paths.clone(), flows, max_seen_flow);
    }
}

fn expand(
    root: &mut Node,
    paths: HashMap<(Label, Label), isize>,
    flows: &HashMap<Label, isize>,
    max_seen_flow: &mut isize,
) {
    let opts = |entity: &Inner| {
        paths
            .iter()
            .filter_map(|((a, b), cost)| (*a == entity.label).then_some((*b, *cost)))
            .filter_map(|(to, travel_cost)| {
                // what's the value of opening this valve for the remainder of the
                // time we have?
                let time_remaining = entity.time_remaining - travel_cost - 1;
                if time_remaining < 0 {
                    // don't have time to visit this node and open the valve.
                    return None;
                }

                let total_value = time_remaining * flows[&to] + entity.total_value;

                Some(Inner {
                    label: to,
                    time_remaining,
                    total_value,
                })
            })
            .collect()
    };

    // Can we calculate some sort of maximal value possible from a given point?
    // eg assume we open all valves instantly, and get the total theoretical
    // value that would provide. There might be branches already above this
    // value?
    //
    // Paths has everyone we've visited already removed, so we just need to find
    // the unique destinations there, find their flows, and multiply by the
    // biggest time remaining.

    let unopened: HashSet<_> = paths.iter().map(|((_, b), _)| *b).collect();
    let max_time = root.me.time_remaining.max(root.ele.time_remaining);
    let potential_extra_flow = max_time * unopened.iter().map(|label| flows[label]).sum::<isize>();

    // Who has the most time remaining? We'll make them move next.
    if root.me.time_remaining > root.ele.time_remaining {
        let me_opts: Vec<Inner> = opts(&root.me);
        root.children = me_opts
            .into_iter()
            .map(|inner| Node {
                me: inner,
                ele: root.ele.clone(),
                children: vec![],
            })
            .filter(|child| {
                let max_flow = child.me.total_value + child.ele.total_value;
                if max_flow > *max_seen_flow {
                    println!("new max: {}", max_flow);
                }

                *max_seen_flow = (*max_seen_flow).max(max_flow);

                let current_flow = child.ele.total_value + child.me.total_value;
                let unrealistic_flow = potential_extra_flow + current_flow;
                unrealistic_flow > *max_seen_flow
            })
            .collect();
        root.children
            .sort_by(|a, b| b.me.total_value.cmp(&a.me.total_value));
    } else {
        let ele_opts: Vec<Inner> = opts(&root.ele);
        root.children = ele_opts
            .into_iter()
            .map(|inner| Node {
                ele: inner,
                me: root.me.clone(),
                children: vec![],
            })
            .filter(|child| {
                let max_flow = child.me.total_value + child.ele.total_value;
                if max_flow > *max_seen_flow {
                    println!("new max: {}", max_flow);
                }
                *max_seen_flow = (*max_seen_flow).max(max_flow);

                let current_flow = child.ele.total_value + child.me.total_value;
                let unrealistic_flow = potential_extra_flow + current_flow;
                unrealistic_flow > *max_seen_flow
            })
            .collect();
        root.children
            .sort_by(|a, b| b.ele.total_value.cmp(&a.ele.total_value));
    }
}

fn shortest_paths(adj: &[(Label, Label)]) -> HashMap<(Label, Label), isize> {
    let labels: HashSet<Label> = adj.iter().flat_map(|(a, b)| [*a, *b]).collect();

    let mut shorts = HashMap::new();

    for start in labels.iter().copied() {
        // Initialise the shortest routes to every node to MAX.
        let mut shortest: HashMap<Label, isize> = labels.iter().map(|l| (*l, isize::MAX)).collect();
        *shortest.get_mut(&start).unwrap() = 0;

        let mut q = VecDeque::<Label>::new();
        q.push_back(start);

        while let Some(current) = q.pop_back() {
            // Get the labels I can get to in a single step from here.
            let next: Vec<Label> = adj
                .iter()
                .filter_map(|(a, b)| (*a == current).then_some(*b))
                .collect();

            for next in next {
                if shortest[&current] + 1 < shortest[&next] {
                    // This path here was better than however we got there
                    // previously. We need to reevaluate all the things it leads
                    // to.
                    *shortest.get_mut(&next).unwrap() = shortest[&current] + 1;
                    q.push_back(next);
                }
            }
        }

        shortest.into_iter().for_each(|(to, cost)| {
            shorts.insert((start, to), cost);
        });
    }

    shorts
}

fn parse(input: &str) -> Input {
    let re = Regex::new(
        r#"Valve (?P<label>[A-Z]{2}) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<to>(?:[A-Z]{2})(?:, (?:[A-Z]{2}))*)"#,
    )
    .unwrap();

    let to_label = |s: &str| {
        assert_eq!(s.len(), 2);
        let s = s.as_bytes();
        Label([s[0] as char, s[1] as char])
    };

    let mut flows = HashMap::new();
    let mut adj = vec![];

    for cap in re.captures_iter(input) {
        let label = to_label(cap.name("label").unwrap().as_str());
        let flow: isize = cap.name("flow").unwrap().as_str().parse().unwrap();
        let to: Vec<_> = cap
            .name("to")
            .unwrap()
            .as_str()
            .split(", ")
            .map(to_label)
            .collect();

        flows.insert(label, flow);
        for to in to {
            // Graph is bidirectional, but that's contained in the input fine.
            adj.push((label, to));
        }
    }

    Input { flows, adj }
}

#[cfg(test)]
mod test {
    use crate::{shortest_paths, Label};
    fn lab(s: &str) -> Label {
        Label::new(s)
    }

    fn pair(a: &str, b: &str) -> (Label, Label) {
        (lab(a), lab(b))
    }

    #[test]
    fn simply_short() {
        let adj = vec![pair("AA", "BB")];
        let sh = shortest_paths(&adj);
        assert_eq!(sh[&pair("AA", "BB")], 1);
    }

    #[test]
    fn shortest_path_more_complex() {
        let adj = vec![
            pair("AA", "BB"),
            pair("BB", "CC"),
            pair("BB", "CX"),
            pair("CX", "CY"),
            pair("CY", "DD"),
            pair("CC", "DD"),
            pair("DD", "EE"),
        ];
        let sh = shortest_paths(&adj);
        assert_eq!(sh[&pair("AA", "EE")], 4);
    }
}
