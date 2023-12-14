use core::panic;

use aoc::line_blocks;
use itertools::Itertools;

fn main() {
    let input = line_blocks("input/day13");
    let part1 = part1(&input);
    dbg!(part1);

    // work low 22166
    // work low 34493
    let part2 = part2(&input);
    dbg!(part2);
}

fn part1(input: &Vec<Vec<String>>) -> usize {
    let mut rows = 0;
    let mut cols = 0;

    for pattern in input {
        let mut match_count = 0;
        let index = find_row_reflection(pattern);
        if let Some(index) = index {
            rows += index + 1;
            match_count += 1;
        }

        let transposed = transpose(pattern);
        let index = find_row_reflection(&transposed);
        if let Some(index) = index {
            cols += index + 1;
            match_count += 1;
        }

        assert!(match_count <= 1);
    }

    // low: 31907
    100 * rows + cols
}

fn part2(input: &Vec<Vec<String>>) -> usize {
    let mut rows = 0;
    let mut cols = 0;

    for pattern in input {
        let mut pattern = pattern.clone();

        if let Some(row) = find_row_reflection(&pattern) {
            // Original reflection is vertical.
            if let Some(index) = find_row_reflection_smudged(&mut pattern, row) {
                // Found a smudge to create a new reflection in the rows
                rows += index + 1;
                continue;
            } else {
                // No new reflection in the vertical, try horizontal.
                let mut transposed = transpose(&pattern);
                let index = find_row_reflection_smudged(&mut transposed, usize::MAX);
                if let Some(index) = index {
                    // Found a smudge to create a new reflection in the cols
                    cols += index + 1;
                    continue;
                } else {
                    panic!("no match");
                }
            }
        }

        let mut transposed = transpose(&pattern);
        if let Some(col) = find_row_reflection(&transposed) {
            // Original reflection is horizontal.
            let index = find_row_reflection_smudged(&mut transposed, col);
            if let Some(index) = index {
                // Found a smudge to create a new reflection in the cols
                cols += index + 1;
                continue;
            } else {
                // need to try smudging other direction.
                let index = find_row_reflection_smudged(&mut pattern, usize::MAX);
                if let Some(index) = index {
                    // Found a smudge to create a new reflection in the rows
                    rows += index + 1;
                    continue;
                } else {
                    panic!("no match");
                }
            }
        }
    }

    100 * rows + cols
}

fn find_row_reflection_smudged(pattern: &mut [String], exclude: usize) -> Option<usize> {
    // Find possible reflection lines ignoring the original.
    let possibles = pattern
        .windows(2)
        .enumerate()
        .filter_map(|(i, rs)| (diffdist(&rs[0], &rs[1]) <= 1 && i != exclude).then_some(i))
        .collect_vec();

    for &possible in &possibles {
        // say matched index 3 of len 6.
        // reflection between index 3 and 6
        // check 3/4, 2/5. so spread 2
        let spread = (possible + 1).min(pattern.len() - possible - 1);
        let mut dist = 0;
        let mut i_fix = usize::MAX;
        for i in 0..spread {
            let this_dist = diffdist(&pattern[possible + i + 1], &pattern[possible - i]);
            dist += this_dist;
            if this_dist == 1 {
                // Fix the rows to look the same.
                i_fix = i;
            }
        }
        if dist == 1 {
            pattern[possible + i_fix + 1] = pattern[possible - i_fix].clone();
            return Some(possible);
        }
    }

    None
}

fn diffdist(a: &str, b: &str) -> usize {
    let mut dist = 0;
    for i in 0..a.len() {
        dist += (a.as_bytes()[i] != b.as_bytes()[i]) as usize;
    }
    dist
}

fn transpose(pattern: &[String]) -> Vec<String> {
    let mut v = vec!["".to_string(); pattern[0].len()];

    for row in pattern {
        for (i, c) in row.chars().enumerate() {
            v[i].push(c);
        }
    }

    v
}

fn find_row_reflection(pattern: &[String]) -> Option<usize> {
    // since it's between rows, there must be two rows that look the same.
    let possibles = pattern
        .windows(2)
        .enumerate()
        .filter_map(|(i, rs)| (rs[0] == rs[1]).then_some(i))
        .collect_vec();

    'outer: for &possible in &possibles {
        // say matched index 3 of len 6.
        // reflection between index 3 and 6
        // check 3/4, 2/5. so spread 2
        let spread = (possible + 1).min(pattern.len() - possible - 1);
        for i in 0..spread {
            if pattern[possible + i + 1] != pattern[possible - i] {
                continue 'outer;
            }
        }

        return Some(possible);
    }

    None
}

#[cfg(test)]
mod test {
    use aoc::line_blocks;

    use crate::*;

    #[test]
    fn test_parse() {
        let patterns = line_blocks("input/day12-example");

        assert_eq!(405, part1(&patterns));
        assert_eq!(400, part2(&patterns));
    }
}
