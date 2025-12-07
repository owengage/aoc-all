use std::{ops::AddAssign, str::FromStr};

use anyhow::{Context, Ok, Result};
use aoc::{fetch_input, text};
use rayon::prelude::*;
use regex::Regex;

const LAST_MINUTE: usize = 32;

#[derive(Debug, Clone, Copy)]
struct Stock {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl AddAssign for Stock {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl Stock {
    fn can_afford(&self, cost: Stock) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn buy(&mut self, cost: Stock) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: usize,
    robots: [BlueprintRobot; 4],
}
impl Blueprint {
    fn max(&self) -> Stock {
        self.robots
            .iter()
            .map(|r| r.cost)
            .reduce(|c1, c2| Stock {
                ore: c1.ore.max(c2.ore),
                clay: c1.clay.max(c2.clay),
                obsidian: c1.obsidian.max(c2.obsidian),
                geode: c1.geode.max(c2.geode),
            })
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct BlueprintRobot {
    robots: Stock,
    cost: Stock,
}

#[derive(Debug, Clone)]
struct Node {
    // previous_bank: Stock,
    bank: Stock,
    robots: Stock,
    elapsed: usize,
    children: Vec<Node>,
}
impl Node {
    fn earn(&mut self) {
        self.bank += self.robots;
    }
}

fn main() {
    // Looks like we have 4 options, making collection robots for: ore, clay,
    // obsidian, geodes. The aim is to maximize geodes in LAST_MINUTE minutes.
    //
    // Is this another tree modelling? At each time step we have multiple
    // options. Does it make sense to try and model each minute separately? Or
    // should we only model making a decision in the tree, and calculate the
    // time?
    //
    // Last time not modelling each minute screwed up. If we're just sitting the
    // tree gets deeper but doesn't really get harder to run computations on.
    let input = parse(&text(fetch_input(2022, 19)));
    let input: Vec<_> = input.into_iter().take(3).collect();

    let results: Vec<_> = input
        .par_iter()
        .map(|bp| {
            let res = quality_level(bp.clone());
            println!("{}, {}", bp.id, res);
            (bp.id, res)
        })
        .collect();

    // println!(
    //     "Overall score: {}",
    //     results.into_iter().map(|(id, q)| id * q).sum::<usize>()
    // );

    // too low: 26796
    // too low: 28014
    println!(
        "Overall score part 2: {}",
        results.into_iter().map(|(_, q)| q).product::<usize>()
    );

    // dbg!(input);
}

fn quality_level(mut bp: Blueprint) -> usize {
    let root = Node {
        // previous_bank: Stock {
        //     ore: 0,
        //     clay: 0,
        //     obsidian: 0,
        //     geode: 0,
        // },
        bank: Stock {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        robots: Stock {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        elapsed: 1,
        children: vec![],
    };

    bp.robots.reverse();

    // What are our options here? What can we afford with minute 1's resources?

    // Deep expand is wrong, we can't build the entire tree. Need a depth first
    // approach so we don't balloon memory.

    fn deep_expand(bp: &Blueprint, root: &Node, mut max_geodes: usize) -> usize {
        if root.elapsed == LAST_MINUTE {
            // if root.robots.geode > 0 {
            //     println!("HAVE GEODE ROBOT {:?}", root);
            // }
            return root.bank.geode;
        }

        let time_left = LAST_MINUTE - root.elapsed;
        if time_left > 2 {
            let theoretical = 1 + (time_left - 1) * (time_left) / 2;
            let theoretical = theoretical + root.bank.geode + time_left * root.robots.geode;
            if theoretical <= max_geodes {
                return max_geodes;
            }
        }

        let children = shallow_expand(bp, root);

        for child in &children {
            max_geodes = max_geodes.max(deep_expand(bp, child, max_geodes));
        }

        // println!("Max geodes so far: {}", max_geodes);
        max_geodes
    }

    deep_expand(&bp, &root, 0)
}

fn shallow_expand(bp: &Blueprint, root: &Node) -> Vec<Node> {
    if root.elapsed > LAST_MINUTE {
        return vec![];
    }

    let mut children = vec![];

    for robot in &bp.robots {
        // If we have enough ore robots to meet *any* robot's ore needs in a single
        // minute, we do not need to make any more of them.
        let max = bp.max().ore;
        let enough_ore_production_for_any = root.robots.ore >= max;

        if robot.robots.ore == 1 && enough_ore_production_for_any {
            continue;
        }

        if let Some(child) = make_child(root, robot.cost, robot.robots) {
            children.push(child);
        }
    }

    let mut do_nothing = root.clone();
    do_nothing.children = vec![];
    do_nothing.earn();
    do_nothing.elapsed += 1;
    children.push(do_nothing);

    children
}

fn make_child(parent: &Node, cost: Stock, robots_made: Stock) -> Option<Node> {
    // Can afford it now, but couldn't last time. Will never be worth delaying..
    // right?

    // Somehow this second condition causes the answer to be higher than without
    // it. This makes very little sense to me on the surface, because we're
    // pruning branhes here. How does this make us find HIGHER values?

    if parent.bank.can_afford(cost) {
        let mut child = parent.clone();
        child.children = vec![];

        // buy, earn, build, update timer.
        child.bank.buy(cost);
        child.earn();
        child.robots += robots_made;

        child.elapsed += 1;
        Some(child)
    } else {
        None
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    // Example:
    // Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 2 ore and 12 obsidian.
    let re = Regex::new(
        r#"Blueprint (\d+): Each ore robot costs (.?+). Each clay robot costs (.?+). Each obsidian robot costs (.?+). Each geode robot costs (.?+)."#,
    ).unwrap();

    let mut bp = vec![];

    for cap in re.captures_iter(input) {
        let id = cap[1].parse().unwrap();
        let ore_robot = cap[2].parse().unwrap();
        let clay_robot = cap[3].parse().unwrap();
        let obsidian_robot = cap[4].parse().unwrap();
        let geode_robot = cap[5].parse().unwrap();

        bp.push(Blueprint {
            id,
            robots: [
                BlueprintRobot {
                    robots: Stock {
                        ore: 1,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    cost: ore_robot,
                },
                BlueprintRobot {
                    robots: Stock {
                        ore: 0,
                        clay: 1,
                        obsidian: 0,
                        geode: 0,
                    },
                    cost: clay_robot,
                },
                BlueprintRobot {
                    robots: Stock {
                        ore: 0,
                        clay: 0,
                        obsidian: 1,
                        geode: 0,
                    },
                    cost: obsidian_robot,
                },
                BlueprintRobot {
                    robots: Stock {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geode: 1,
                    },
                    cost: geode_robot,
                },
            ],
        });
    }

    bp
}

impl FromStr for Stock {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(" and ")
            .flat_map(|quant| -> Result<_> {
                let (val, unit) = quant.split_once(' ').context("invalid quantity")?;
                let val = val.parse::<usize>()?;
                match unit {
                    "ore" => Ok(Stock {
                        ore: val,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    }),
                    "clay" => Ok(Stock {
                        ore: 0,
                        clay: val,
                        obsidian: 0,
                        geode: 0,
                    }),
                    "obsidian" => Ok(Stock {
                        ore: 0,
                        clay: 0,
                        obsidian: val,
                        geode: 0,
                    }),
                    _ => anyhow::bail!("invalid unit"),
                }
            })
            .reduce(|a, b| Stock {
                ore: a.ore + b.ore,
                clay: a.clay + b.clay,
                obsidian: a.obsidian + b.obsidian,
                geode: a.geode + b.geode,
            })
            .context("invalid cost")
    }
}
