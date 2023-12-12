use aoc::lines;

fn main() {
    // ???.### 1,1,3 - 1 arrangement
    // .??..??...?##. 1,1,3 - 4 arrangements
    // ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    // ????.#...#... 4,1,1 - 1 arrangement
    // ????.######..#####. 1,6,5 - 4 arrangements
    // ?###???????? 3,2,1 - 10 arrangements

    // Max field size is only 20 characters.
    let input = lines("input/day12");

    // How to store this? Three states, arbitrary length, and a list of numbers.
    // We always know what the damaged (#) springs look like as blocks. eg
    // ???.### 1,1,3 must be #, #, ### in that order. We need to fit those in.
    //
    // First we'll put the values in as left as we can...
    // #.#.###
    // We need to remember the fixed input (the last three here).
    // Can we move any of the none fixed groups left? If no, we're done.
    // What about: ?###???????? 3,2,1
    // Placement:  .###.##.#???. Move left to right, if the ? gap is big enough
    // How to programmatically replace the ?'s?
    // With only ~10 binary options, could probably brute force. Part 2 may
    // screw that up.
}

fn part1(lines: &[String]) -> usize {
    // Plan of attack:
    // - Generate all possible arrangements of a given size and list of damaged
    //   springs
    // - For each, validate if it fits the given pattern.

    todo!();
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let (field, spec) = line.split_once(' ').unwrap();
    (
        field.chars().collect(),
        spec.split(',').map(|s| s.parse().unwrap()).collect(),
    )
}

struct FieldIter(Vec<char>);

impl FieldIter {
    fn new(size: usize, spec: &[usize]) -> Self {
        let mut field = vec![];
        for &s in spec {
            field.resize(field.len() + s, '#');
            field.push('.');
        }
        field.pop();

        assert!(field.len() <= size);

        while field.len() < size {
            field.push('.');
        }

        Self(field)
    }
}

impl Iterator for FieldIter {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        // eg ...#..##...#..
        // to ...#..##....#.
        // want to move right most right one.
        if self.0.is_empty() {
            return None;
        }

        let ret = self.0.clone();

        let f = &mut self.0;
        let mut i = f.len() - 1;

        //            v
        // ...#..##...#..
        while f[i] == '#' {
            i -= 1;
        }
        while f[i] == '.' {
            i -= 1;
        }

        if i == f.len() - 1 {
            f.clear(); // done
            return Some(ret);
        }

        // Move the #s
        while f[i] == '#' {
            f.swap(i, i + 1);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        return Some(ret);

        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_make_iter() {
        assert_eq!(
            "#.#".chars().collect::<Vec<_>>(),
            FieldIter::new(3, &[1, 1]).0
        );
        assert_eq!(
            "#.####.##...".chars().collect::<Vec<_>>(),
            FieldIter::new(12, &[1, 4, 2]).0
        );
    }

    #[test]
    fn test_next() {
        let mut iter = FieldIter::new(3, &[1]);
        assert_next("#..", &mut iter);
        assert_next(".#.", &mut iter);
        assert_next("..#", &mut iter);
        assert_next_none(&mut iter);
    }

    #[test]
    fn test_double() {
        let mut iter = FieldIter::new(3, &[2]);
        assert_next("##.", &mut iter);
        assert_next(".##", &mut iter);
        assert_next_none(&mut iter);
    }

    #[test]
    fn test_large() {
        let mut iter = FieldIter::new(10, &[5]);
        assert_next("#####.....", &mut iter);
        assert_next(".#####....", &mut iter);
        assert_next("..#####...", &mut iter);
        // assert_next_none(&mut iter);
    }

    #[test]
    fn test_multi() {
        let mut iter = FieldIter::new(6, &[1, 1]);
        assert_next("#.#...", &mut iter);
        assert_next("#..#..", &mut iter);
        assert_next("#...#.", &mut iter);
        assert_next("#....#", &mut iter);
        assert_next(".#...#", &mut iter);
        assert_next("..#..#", &mut iter);
        assert_next("...#.#", &mut iter);
        assert_next_none(&mut iter);
    }

    fn assert_next(field: &str, iter: &mut FieldIter) {
        assert_eq!(field.chars().collect::<Vec<_>>(), iter.next().unwrap());
    }

    fn assert_next_none(iter: &mut FieldIter) {
        assert!(iter.next().is_none());
    }
}
