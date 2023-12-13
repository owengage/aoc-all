use std::collections::VecDeque;

use aoc::lines;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    // ???.### 1,1,3 - 1 arrangement
    // .??..??...?##. 1,1,3 - 4 arrangements
    // ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    // ????.#...#... 4,1,1 - 1 arrangement
    // ????.######..#####. 1,6,5 - 4 arrangements
    // ?###???????? 3,2,1 - 10 arrangements

    // Max field size is only 20 characters.
    let input = lines("input/work12");
    let arrs: Vec<_> = input.iter().map(|s| parse_line(s)).collect();

    let part1 = count_all_arrangements(arrs.clone());

    let arrs: Vec<_> = arrs
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
    let part2 = count_all_arrangements(arrs.clone());

    dbg!(part1);
    dbg!(part2);
}

fn count_all_arrangements(arrs: Vec<(Vec<char>, Vec<usize>)>) -> usize {
    arrs.iter()
        .map(|(field, spec)| {
            let count = possible_arrangements(field, spec);
            println!("Done ({count}): {:?}, {:?}", field, spec);
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

    // That last block might be larger than the spec'd block.
    if found_unknown && block_len > 0 {
        let next_in_spec = spec.get(blocks.len());
        if next_in_spec.is_some() && block_len > *next_in_spec.unwrap() {
            return false;
        }

        if next_in_spec.is_none() {
            return false;
        }
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

#[cfg(test)]
mod test {

    use itertools::Itertools;

    use crate::*;

    #[test]
    fn ans() {
        let input = lines("input/work12");
        let arrs: Vec<_> = input.iter().map(|s| parse_line(s)).collect();
        assert_eq!(7857, count_all_arrangements(arrs));
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
