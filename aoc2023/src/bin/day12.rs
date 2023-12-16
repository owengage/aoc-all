use core::panic;
use std::{collections::VecDeque, sync::atomic::AtomicI32};

use aoc::lines;

fn main() {
    // ???.### 1,1,3 - 1 arrangement
    // .??..??...?##. 1,1,3 - 4 arrangements
    // ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    // ????.#...#... 4,1,1 - 1 arrangement
    // ????.######..#####. 1,6,5 - 4 arrangements
    // ?###???????? 3,2,1 - 10 arrangements

    // Alternative idea:
    //
    // We use recursion rather than a queue. The stack is only ever going to be
    // about 100 deep since that's the maximum length of an input.
    //
    // But also to memoise the tails that we see. Maybe for all input, maybe for
    // each. The recursion function is then working only on the tails. This is
    // made easier by only ever adding full blocks including the dividing dot.
    // This measn the function just accepts the remaining input and the
    // remaining specification.

    // Max field size is only 20 characters.
    let input = lines("aoc2023/input/work12");

    let arrs: Vec<_> = input.iter().map(|s| parse_line(s)).collect();
    let part1: usize = arrs
        .iter()
        .map(|(f, sp)| count_tail_arrangements(f, sp))
        .sum();

    let long_arrs: Vec<_> = arrs
        .into_iter()
        .map(|(f, s)| {
            let s_len = s.len();
            let s: Vec<_> = s.into_iter().cycle().take(s_len * 5).collect();
            let f = {
                let mut all = vec![];
                for _ in 0..5 {
                    all.extend_from_slice(&f);
                    all.push('?');
                }
                all.pop();
                all
            };

            (f, s)
        })
        .collect();

    let part2: usize = long_arrs
        .iter()
        .enumerate()
        .map(|(i, (f, sp))| {
            println!("Doing {i}");
            let count = count_tail_arrangements(f, sp);
            println!("Done {i}");
            count
        })
        .sum();

    dbg!(part1);
    dbg!(part2);
}

fn count_tail_arrangements(tail: &[char], spec: &[usize]) -> usize {
    let (matches, tail, spec) = skip_known(tail, spec);
    if !matches {
        return 0;
    }

    if tail.is_empty() && spec.is_empty() {
        return 1;
    }

    let mut total = 0;

    // Can we place the next block here?
    let mut tail_block = tail.to_vec();
    let mut tail_dot = tail.to_vec();

    if !spec.is_empty() {
        let placed = place_block(&mut tail_block, spec[0]);

        if placed {
            let Some(new_tail) = tail_block.get(spec[0] + 1..) else {
                // we must have completed the entire tail.
                if matches_spec(&tail_block, spec) {
                    return 1;
                } else {
                    return 0;
                }
            };

            total += count_tail_arrangements(new_tail, &spec[1..]);
        }
    }

    // Can we place a dot?
    total += match tail_dot.first() {
        Some('#') => 0,
        Some('.' | '?') => {
            tail_dot[0] = '.';
            count_tail_arrangements(&tail_dot[1..], spec)
        }
        None => 0,
        _ => panic!(),
    };

    total
}

fn skip_known<'t, 's>(tail: &'t [char], mut spec: &'s [usize]) -> (bool, &'t [char], &'s [usize]) {
    let mut current_block = 0;
    let mut last_complete_block = 0;

    for i in 0..tail.len() {
        match tail[i] {
            '#' => {
                current_block += 1;
            }
            '.' => {
                if current_block != 0 {
                    if spec.is_empty() {
                        return (false, tail, spec);
                    }
                    if spec[0] != current_block {
                        // we don't match the spec!
                        return (false, tail, spec);
                    } else {
                        // Complete this block.
                        // What happens if this is the end of a block, we return
                        // the tail ##.?, we return .? rather than just ?
                        spec = &spec[1..];
                        last_complete_block = i + 1; // include the dot.
                        current_block = 0;
                    }
                }
            }
            '?' => return (true, &tail[last_complete_block..], spec),
            _ => panic!(),
        }
    }

    if current_block != 0 {
        if spec.is_empty() {
            return (false, tail, spec);
        }
        if spec[0] != current_block {
            // we don't match the spec!
            (false, tail, spec)
        } else {
            spec = &spec[1..];
            (true, &tail[tail.len()..], spec)
        }
    } else {
        (true, &tail[last_complete_block..], spec)
    }
}

// Try to place the given block immediately. Return true is successful.
fn place_block(tail: &mut [char], block: usize) -> bool {
    for i in 0..block {
        match tail.get(i) {
            Some('#' | '?') => {
                tail[i] = '#';
            }
            Some('.') => return false,
            None => return false,
            _ => panic!(),
        }
    }

    // Now need to place last dot to finish block.
    match tail.get(block) {
        Some('#') => false,
        Some('.') => true,
        Some('?') => {
            tail[block] = '.';
            true
        }
        None => true,
        _ => panic!(),
    }
}

fn count_all_arrangements(arrs: Vec<(Vec<char>, Vec<usize>)>) -> usize {
    let done = AtomicI32::new(0);

    arrs.iter()
        .map(|(field, spec)| {
            println!("Doing: {:?}, {:?}", field, spec);
            let count = possible_arrangements(field, spec);
            done.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

            println!(
                "Done {} ({count}): {:?}, {:?}",
                done.load(std::sync::atomic::Ordering::SeqCst),
                field,
                spec
            );
            count
        })
        .sum()
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let (field, spec) = line.split_once(' ').unwrap();
    (
        field.chars().collect(),
        spec.split(',').map(|s| s.parse().unwrap()).collect(),
    )
}

#[derive(Debug, Clone)]
struct Node {
    field: Vec<char>,
}

fn possible_arrangements(field: &[char], spec: &[usize]) -> usize {
    let root = Node {
        field: field.to_vec(),
    };
    let mut nodes = VecDeque::new();
    nodes.push_back(root);
    let mut count = 0;

    while let Some(node) = nodes.pop_back() {
        let complete = expand_node(&mut nodes, &node, spec);
        if complete && matches_spec(&node.field, spec) {
            // println!("Matched: {:?}, {:?}", node.field, spec);
            count += 1;
        }
    }

    count
}

fn matches_spec(field: &[char], spec: &[usize]) -> bool {
    let mut block_len = 0;
    let mut blocks = vec![];

    for &spring in field {
        if spring == '#' {
            block_len += 1;
        } else if block_len > 0 {
            blocks.push(block_len);
            block_len = 0;
        }
    }

    if block_len > 0 {
        blocks.push(block_len);
    }

    blocks.eq(spec)
}

fn expand_node(nodes: &mut VecDeque<Node>, node: &Node, spec: &[usize]) -> bool {
    // if partial_matches_full_spec(&node.field, spec) {
    //     return true;
    // }

    // At this point, rather than set just one character, can we set multiple?
    // If we need a block of 4 # next and we know the last block is complete, we
    // can set all 4 simultaneously.

    let unknown = find_unknown(&node.field);
    if let Some(unknown) = unknown {
        let mut field = node.field.clone();
        field[unknown] = '.';
        if matches_spec_partial(&field, spec) {
            nodes.push_back(Node { field });
        }

        //
        // Add node for the next entire block of springs.
        //

        let mut field = node.field.clone();
        // Can we put the next block down here? Or complete the current block?
        let num_springs_required = compl_block(&field, spec);
        if num_springs_required == 0 {
            return false;
        }

        if num_springs_required + unknown > field.len() {
            // ...?...
            return false;
        }
        for i in 0..num_springs_required {
            if field[unknown + i] == '?' || field[unknown + i] == '#' {
                field[unknown + i] = '#';
            } else {
                return false;
            }
        }
        // field[unknown] = '#';

        if matches_spec_partial(&field, spec) {
            nodes.push_back(Node { field });
        }

        false
    } else {
        // we're a complete node.
        true
    }
}

fn compl_block(field: &[char], spec: &[usize]) -> usize {
    let mut block_len = 0;
    let mut blocks = vec![];
    let mut last_block_unknown = false;

    for (_i, &spring) in field.iter().enumerate() {
        if spring == '?' {
            last_block_unknown = true;
            break;
        }
        if spring == '#' {
            block_len += 1;
        } else if block_len > 0 {
            blocks.push(block_len);
            block_len = 0;
        }
    }

    if !last_block_unknown && block_len > 0 {
        blocks.push(block_len);

        if let Some(s) = spec.get(blocks.len()) {
            return *s;
        }
    } else if let Some(expected_block_size) = spec.get(blocks.len()) {
        return expected_block_size - block_len;
    }

    1
}

fn matches_spec_partial(field: &[char], spec: &[usize]) -> bool {
    let mut block_len = 0;
    let mut blocks = vec![];
    let mut found_unknown = false;
    let mut last_complete_block_i = 0;

    for (i, &spring) in field.iter().enumerate() {
        if spring == '?' {
            found_unknown = true;
            break;
        }
        if spring == '#' {
            block_len += 1;
        } else if block_len > 0 {
            last_complete_block_i = i;
            blocks.push(block_len);
            block_len = 0;
        }
    }

    if !found_unknown && block_len > 0 {
        blocks.push(block_len);
    }

    if blocks.len() > spec.len() {
        return false;
    }

    let eq_so_far = spec
        .iter()
        .take(blocks.len())
        .copied()
        .eq(blocks.iter().copied());

    if !eq_so_far {
        return false;
    }

    // Idea: Ones that are already complete according to spec. Doesn't seem to
    // make much difference.

    // // That last block might be larger than the spec'd block.
    // if found_unknown && block_len > 0 {
    //     let next_in_spec = spec.get(blocks.len());
    //     if next_in_spec.is_some() && block_len > *next_in_spec.unwrap() {
    //         return false;
    //     }

    //     if next_in_spec.is_none() {
    //         return false;
    //     }
    // }

    // Can we possibly fit the remaining blocks? If not no point exploring the
    // branch.
    if !could_fit(field, spec) {
        return false;
    }

    // How many unknowns do we still need in order to be able to fit things in?
    let remaining_blocks: Vec<_> = spec.iter().skip(blocks.len()).collect();
    if !remaining_blocks.is_empty() {
        // We need at least all the springs number of chars left, plus at least
        // 1 dot between each one.
        let left: usize =
            remaining_blocks.iter().copied().sum::<usize>() + remaining_blocks.len() - 1;
        let max_remaining_places = field.len() - last_complete_block_i;
        if max_remaining_places < left {
            return false; // can't fit un the rest of the blocks.
        }
    }

    true
}

fn partial_matches_full_spec(field: &[char], spec: &[usize]) -> bool {
    let mut block_len = 0;
    let mut blocks = vec![];
    let mut last_block_unknown = false;

    for (i, &spring) in field.iter().enumerate() {
        if spring == '?' {
            last_block_unknown = true;
            break;
        }
        if spring == '#' {
            block_len += 1;
        } else if block_len > 0 {
            blocks.push(block_len);
            block_len = 0;
        }
    }

    if !last_block_unknown && block_len > 0 {
        blocks.push(block_len);
    }

    // compare against full spec
    spec.iter().copied().eq(blocks.iter().copied())
}

fn find_unknown(field: &[char]) -> Option<usize> {
    for (i, &c) in field.iter().enumerate() {
        if c == '?' {
            return Some(i);
        }
    }

    None
}

fn could_fit(field: &[char], mut spec: &[usize]) -> bool {
    let mut i = 0;
    loop {
        let Some(&s) = spec.first() else {
            // Ran out of spec, must fit!
            return true;
        };

        let Some(slice) = &field.get(i..i + s) else {
            // Don't have enough string left to fit the block!
            return false;
        };

        if slice.iter().all(|&c| c == '#' || c == '?') {
            // block fits
            // Either we're at the end of the string, or the next character must
            // be ? or .
            if let Some(&dot) = field.get(i + s) {
                if dot == '#' {
                    // would make us part of another block, we don't fit!
                    i += 1;
                    continue;
                }
                i += slice.len() + 1;
            } else {
                // end of string.
                i += slice.len();
            }

            spec = &spec[1..];
        } else {
            // Doesn't fit, let's move one over.
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {

    use itertools::Itertools;

    use crate::*;

    #[test]
    fn fit() {
        assert!(could_fit(&['?'], &[1]));
        assert!(could_fit(&['.', '.', '?'], &[1]));
        assert!(could_fit(&['.', '.', '?', '?'], &[2]));
        assert!(could_fit(&['.', '?', '.', '?', '?'], &[2]));
        assert!(could_fit(&['.', '?', '.', '#', '?'], &[2]));
        assert!(could_fit(&['.', '?', '.', '#', '#'], &[2]));
        assert!(could_fit(&['.', '?', '.', '#', '#'], &[1, 2]));
        assert!(could_fit(&['.', '?', '.', '#', '#', '?', '?'], &[1, 2, 1]));

        assert!(!could_fit(&['.', '?', '.', '#', '#', '?'], &[1, 2, 1]));
        assert!(!could_fit(&['?', '.', '?', '?'], &[2, 1]));
        assert!(!could_fit(&['.', '.', '?'], &[2]));
        assert!(!could_fit(&['.'], &[1]));
    }

    #[test]
    fn new_way() {
        assert_eq!(1, count_tail_arrangements(&['?', '.', '#'], &[1, 1]));
        assert_eq!(1, count_tail_arrangements(&['?', '?', '#'], &[1, 1]));
        assert_eq!(1, count_tail_arrangements(&['?', '?', '?'], &[1, 1]));
        assert_eq!(2, count_tail_arrangements(&['?', '?', '?'], &[2]));
        assert_eq!(2, count_tail_arrangements(&['?', '#', '?'], &[2]));
        assert_eq!(
            3,
            count_tail_arrangements(&['?', '?', '#', '#', '?', '?'], &[4])
        );

        assert_eq!(
            5,
            count_tail_arrangements(
                &[
                    '?', '?', '?', '?', '?', '?', '?', '#', '#', '?', '?', '?', '?', '?', '#', '?',
                    '#', '?'
                ],
                &[9, 6]
            )
        );

        assert_eq!(
            4,
            count_tail_arrangements(&"????.######..#####.".chars().collect_vec(), &[1, 6, 5])
        );
        assert_eq!(
            10,
            count_tail_arrangements(&"?###????????".chars().collect_vec(), &[3, 2, 1])
        );
        assert_eq!(
            1,
            count_tail_arrangements(&"?#?#?#?#?#?#?#?".chars().collect_vec(), &[1, 3, 1, 6])
        );
        assert_eq!(
            4,
            count_tail_arrangements(&"??????#.??".chars().collect_vec(), &[2, 2]) // ??????#.??
                                                                                  // ##...##...
                                                                                  // .##..##...
                                                                                  // ..##.##...
                                                                                  // .....##.##
        );
    }

    #[test]
    fn fuzz() {
        let field = "##.??.....????####???##?".chars().collect_vec();
        let spec = &[2, 2, 5];
        let expected = possible_arrangements(&field, spec);
        let actual = count_tail_arrangements(&field, spec);
        assert_eq!(expected, actual);
        // Try example input.
    }

    #[test]
    fn ans() {
        let input = lines("input/work12");
        let arrs: Vec<_> = input.iter().map(|s| parse_line(s)).collect();
        let ans: usize = arrs
            .iter()
            .map(|(f, sp)| count_tail_arrangements(f, sp))
            .sum();

        assert_eq!(7857, ans);
    }

    #[test]
    fn test_single_unknown() {
        assert_eq!(1, possible_arrangements(&"..?".chars().collect_vec(), &[1]))
    }

    #[test]
    fn matcher() {
        assert!(matches_spec(&"..#".chars().collect_vec(), &[1]));
        assert!(matches_spec(&"#.#".chars().collect_vec(), &[1, 1]));
        assert!(matches_spec(&"##.#".chars().collect_vec(), &[2, 1]));
        assert!(matches_spec(&"##.#".chars().collect_vec(), &[2, 1]));
        assert!(matches_spec(
            &"####.....#...###...#..".chars().collect_vec(),
            &[4, 1, 3, 1]
        ));
    }

    #[test]
    fn test_num_springs() {
        assert_eq!(1, compl_block(&".#???".chars().collect_vec(), &[2]));
        assert_eq!(2, compl_block(&".#???".chars().collect_vec(), &[3]));
        assert_eq!(3, compl_block(&"..???".chars().collect_vec(), &[3]));
        assert_eq!(3, compl_block(&"#.???".chars().collect_vec(), &[1, 3]));
        assert_eq!(2, compl_block(&"#.#??..".chars().collect_vec(), &[1, 3]));
        assert_eq!(
            1,
            compl_block(&"....???....".chars().collect_vec(), &[1, 3])
        );
        assert_eq!(4, compl_block(&"..#?...".chars().collect_vec(), &[5]));
        assert_eq!(5, compl_block(&"...?...".chars().collect_vec(), &[5]));
        assert_eq!(1, compl_block(&".##.?...".chars().collect_vec(), &[2, 1]));
        assert_eq!(
            2,
            compl_block(&"##.??.??##?".chars().collect_vec(), &[2, 2, 4])
        );
    }

    #[test]
    fn test_example() {
        let arr = possible_arrangements(&"##.??.??##?".chars().collect_vec(), &[2, 2, 4]);
    }
}
