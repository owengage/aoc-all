use core::panic;
use std::collections::HashSet;

use aoc::{
    fetch_input, lines,
    two::{pt, IPoint},
};

// What if I instead made all options to get from X to Y and passed those up and
// up, finally selecting the last one? Will part 2 screw me? Maybe just x then
// y.

// That worked but now there's 25 robots! I think I'll need to resort to a tree
// search and do tree pruning. If I do depth first I can prune any branches that
// end up longer than the current shortest code.

fn main() {
    let codes = lines(fetch_input(2024, 21));
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+

    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+

    // Given a code to type on the top pad, what directions do I need on the
    // bottom one? Try and make this in such a way that I can generalise to a
    // pad of a pad next.

    let code = &codes[0];
    let mut part1 = 0;

    for code in codes {
        let moves_for_keypad = shortest_for_keypad(&code);

        let mut opts = moves_for_keypad;

        for _ in 0..2 {
            let mut new_opts = HashSet::new();
            for opt in &opts {
                new_opts.extend(shortest_for_dirpad(opt));
            }
            opts = new_opts;
        }

        let shortest = opts.iter().min_by_key(|s| s.len()).unwrap();

        let num: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
        let num: usize = num.parse().unwrap();
        let complexity = shortest.len() * num;
        part1 += complexity;
        println!("{code}, {complexity}: {shortest}");
    }

    assert_eq!(part1, 157892);
    println!("part1 = {part1}");
}

fn shortest_for_keypad(code: &str) -> HashSet<String> {
    let mut it = code.chars();
    let mut current = it.next().unwrap();
    let mut moves: HashSet<String> = moves_keypad('A', current).into_iter().collect();

    for ch in it {
        let m = moves_keypad(current, ch);
        current = ch;

        let mut new_moves = HashSet::new();

        for prev in moves {
            for ap in &m {
                new_moves.insert(prev.clone() + ap.as_str());
            }
        }

        moves = new_moves;
    }

    moves
}

fn shortest_for_dirpad(code: &str) -> HashSet<String> {
    let mut it = code.chars();
    let mut current = it.next().unwrap();
    let mut moves: HashSet<String> = moves_dirpad('A', current).into_iter().collect();

    for ch in it {
        let m = moves_dirpad(current, ch);
        current = ch;

        let mut new_moves = HashSet::new();

        for prev in moves {
            for ap in &m {
                new_moves.insert(prev.clone() + ap.as_str());
            }
        }

        moves = new_moves;
    }

    moves
}

fn moves_keypad(current: char, dest: char) -> Vec<String> {
    let mut moves = vec![];
    let start = keypad_to_point(current);
    let end = keypad_to_point(dest);

    if start.y == 3 && end.x == 0 {
        let mut m1 = String::new();
        movey(&mut m1, &end.y, &start.y);
        movex(&mut m1, &end.x, &start.x);
        moves.push(m1 + "A");
    } else {
        let mut m = String::new();
        movex(&mut m, &end.x, &start.x);
        movey(&mut m, &end.y, &start.y);
        moves.push(m + "A");

        let mut m = String::new();
        movey(&mut m, &end.y, &start.y);
        movex(&mut m, &end.x, &start.x);
        moves.push(m + "A");
    }

    moves
}

fn moves_dirpad(current: char, dest: char) -> Vec<String> {
    let mut moves = vec![];
    let start = dirpad_to_point(current);
    let end = dirpad_to_point(dest);

    if start.y == 0 && end.x == 0 {
        let mut m1 = String::new();
        movey(&mut m1, &end.y, &start.y);
        movex(&mut m1, &end.x, &start.x);
        moves.push(m1 + "A");
    } else {
        let mut m = String::new();
        movex(&mut m, &end.x, &start.x);
        movey(&mut m, &end.y, &start.y);
        moves.push(m + "A");

        let mut m = String::new();
        movey(&mut m, &end.y, &start.y);
        movex(&mut m, &end.x, &start.x);
        moves.push(m + "A");
    }

    moves
}

fn movex(moves: &mut String, end_x: &isize, start_x: &isize) {
    let dx = *end_x - *start_x;
    if dx >= 0 {
        for _ in 0..dx {
            moves.push('>');
        }
    } else {
        for _ in 0..-dx {
            moves.push('<');
        }
    }
}

fn movey(moves: &mut String, end_y: &isize, start_y: &isize) {
    let dy = *end_y - *start_y;
    if dy >= 0 {
        for _ in 0..dy {
            moves.push('v');
        }
    } else {
        for _ in 0..-dy {
            moves.push('^');
        }
    }
}

fn keypad_to_point(dest: char) -> IPoint {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+

    // Try 3 to 5
    //  ^<A      or                 <^A
    //  <Av<A>>^A                   v<<A>^A>A
    //  v<<A>>^Av<A<A>>^AvAA<^A>A   v<A<AA>>^AvA<^A>AvA^A
    //
    //  v<<A>>^A<vA<A>>^AvAA<^A>A   v<A<AA>>^AvA<^A>AvA^A
    //
    // above shows to stick to a given direction as much as possible.
    //

    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    match dest {
        '7' => pt(0, 0),
        '8' => pt(1, 0),
        '9' => pt(2, 0),
        '4' => pt(0, 1),
        '5' => pt(1, 1),
        '6' => pt(2, 1),
        '1' => pt(0, 2),
        '2' => pt(1, 2),
        '3' => pt(2, 2),
        '0' => pt(1, 3),
        'A' => pt(2, 3),
        _ => panic!(),
    }
}

fn dirpad_to_point(dest: char) -> IPoint {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    match dest {
        '^' => pt(1, 0),
        'A' => pt(2, 0),
        '<' => pt(0, 1),
        'v' => pt(1, 1),
        '>' => pt(2, 1),
        _ => panic!(),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        dirpad_to_point, keypad_to_point, moves_dirpad, moves_keypad, shortest_for_dirpad,
        shortest_for_keypad,
    };

    #[test]
    fn bad() {
        let moves = moves_keypad('3', '5') + "A";
        let dir1 = shortest_for_dirpad(&moves);
        let dir2 = shortest_for_dirpad(&dir1);
        println!("{moves}");
        println!("{dir1}");
        println!("{dir2}");
        assert_eq!(dir2.len(), 21);
    }

    #[test]
    fn keypad() {
        assert_eq!(moves_keypad('A', 'A'), "");
        assert_eq!(moves_keypad('A', '9'), "^^^");
        assert_eq!(moves_keypad('9', 'A'), "vvv");
        assert_eq!(moves_keypad('1', '7'), "^^");
        assert_eq!(moves_keypad('1', '3'), ">>");
        assert_eq!(moves_keypad('5', '4'), "<");
        assert_eq!(moves_keypad('1', '9'), ">>^^");
        assert_eq!(moves_keypad('1', '0'), ">v");
        assert_eq!(moves_keypad('0', '1'), "^<");
        assert_eq!(moves_keypad('A', '1'), "^<<");
        assert_eq!(moves_keypad('A', '7'), "^^^<<");
        assert_eq!(shortest_for_keypad("593A"), "^^<A>^AvvAvA");

        for src in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'] {
            for dst in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'] {
                let sp = keypad_to_point(src);
                let dp = keypad_to_point(dst);
                assert_eq!(sp.taxicab_dist(dp), moves_keypad(src, dst).len());
            }
        }
    }

    #[test]
    fn dirpad() {
        assert_eq!(moves_dirpad('A', '^'), "<");
        assert_eq!(moves_dirpad('A', 'v'), "v<");
        assert_eq!(moves_dirpad('A', '<'), "v<<");
        assert_eq!(moves_dirpad('<', 'A'), ">>^");

        for src in ['A', '^', '<', '>'] {
            for dst in ['A', '^', '<', '>'] {
                let sp = dirpad_to_point(src);
                let dp = dirpad_to_point(dst);
                assert_eq!(sp.taxicab_dist(dp), moves_dirpad(src, dst).len());
            }
        }
        assert_eq!(shortest_for_keypad("593A"), "^^<A>^AvvAvA");
    }

    #[test]
    fn example() {
        println!("379A");
        println!("{}", shortest_for_keypad("379A"));
        println!("{}", shortest_for_dirpad(&shortest_for_keypad("379A")));
        println!(
            "{}",
            shortest_for_dirpad(&shortest_for_dirpad(&shortest_for_keypad("379A")))
        );

        assert_eq!(
            shortest_for_dirpad(&shortest_for_dirpad(&shortest_for_keypad("379A"))),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
        )
    }
}
