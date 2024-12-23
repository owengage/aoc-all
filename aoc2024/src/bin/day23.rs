use std::collections::{HashMap, HashSet};

use aoc::{fetch_input, lines};
use itertools::Itertools;

type Node = [char; 2];

fn main() {
    let input = lines(fetch_input(2024, 23));
    let adj = parse_adj(input);
    let adj = {
        let mut map = HashMap::<Node, HashSet<Node>>::new();
        for (a, b) in adj.into_iter() {
            map.entry(a).or_default().insert(b);
        }
        map
    };

    let mut triples = HashSet::<[Node; 3]>::new();

    for (node, connected1) in &adj {
        for player2 in connected1 {
            let connected2 = adj.get(player2).unwrap();
            // find someone in connected2 that's also in connected1
            for player3 in connected2 {
                if adj[player3].contains(node) {
                    let mut ordered = [*node, *player2, *player3];
                    ordered.sort();
                    triples.insert(ordered);
                }
            }
        }
    }
    let triples_with_t = triples
        .iter()
        .filter(|trip| trip.iter().any(|n| n[0] == 't'))
        .count();

    println!("part1 = {}", triples_with_t);

    let mut max_clique = HashSet::new();
    let mut handle = |clique: &HashSet<Node>| {
        if clique.len() > max_clique.len() {
            max_clique = clique.clone();
        }
    };

    bron_kerbosch(
        &adj,
        HashSet::new(),
        adj.keys().cloned().collect(),
        HashSet::new(),
        &mut handle,
    );

    let mut mc = max_clique.into_iter().collect_vec();
    mc.sort();

    println!(
        "part2 = {}",
        mc.into_iter().map(String::from_iter).join(",")
    );
}

fn bron_kerbosch(
    adj: &HashMap<Node, HashSet<Node>>,
    r: HashSet<Node>,
    mut p: HashSet<Node>,
    mut x: HashSet<Node>,
    handle: &mut impl FnMut(&HashSet<Node>),
) {
    if p.is_empty() && x.is_empty() {
        // r is maximal clique.
        handle(&r);
    }

    while !p.is_empty() {
        let v = *p.iter().next().unwrap();

        let mut r = r.clone();
        r.insert(v);
        bron_kerbosch(
            adj,
            r,
            p.intersection(&adj[&v]).cloned().collect(),
            x.intersection(&adj[&v]).cloned().collect(),
            handle,
        );
        p.remove(&v);
        x.insert(v);
    }
}

fn parse_adj(input: Vec<String>) -> Vec<(Node, Node)> {
    let mut adj = vec![];
    for line in input {
        let (a, b) = line.split_once("-").unwrap();
        assert_eq!(2, a.len());
        assert_eq!(2, b.len());
        let a = a.chars().collect_vec().try_into().unwrap();
        let b = b.chars().collect_vec().try_into().unwrap();
        adj.push((a, b));
        adj.push((b, a));
    }

    adj
}
