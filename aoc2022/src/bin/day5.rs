use std::{collections::VecDeque, io::BufRead};

use aoc::{fetch_input, text};

#[derive(Debug, PartialEq)]
struct Move {
    count: i32,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct State {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

fn parse_input<R: BufRead>(mut reader: R) -> State {
    let mut stacks = vec![];

    (&mut reader)
        .lines()
        .flatten()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let line = line.as_bytes();
            let mut n = 0;
            loop {
                let i = 4 * n + 1;
                if i >= line.len() {
                    break;
                }
                while stacks.len() < n + 1 {
                    stacks.push(VecDeque::new());
                }
                if line[i].is_ascii_digit() {
                    break; // end of stack input, hacky
                }
                if line[i] != b' ' {
                    let c = line[i] as char;
                    stacks[n].push_front(c);
                }
                n += 1;
            }
        });

    let moves = reader
        .lines()
        .flatten()
        .map(|line| {
            let comp: Vec<_> = line.split(' ').collect();
            assert_eq!(comp.len(), 6);
            Move {
                count: comp[1].parse().unwrap(),
                from: comp[3].parse::<usize>().unwrap() - 1,
                to: comp[5].parse::<usize>().unwrap() - 1,
            }
        })
        .collect();

    State { stacks, moves }
}

fn main() {
    let input = text(fetch_input(2022, 5));
    let mut state = parse_input(input.as_bytes());

    for mov in state.moves {
        let mut tmp = vec![];
        for _ in 0..mov.count {
            // // PART 1
            // let crat = state.stacks[mov.from].pop_back().unwrap(); // should be a crate there.
            // state.stacks[mov.to].push_back(crat);

            // PART 2
            let crat = state.stacks[mov.from].pop_back().unwrap(); // should be a crate there.
            tmp.push(crat);
        }

        for _ in 0..mov.count {
            let crat = tmp.pop().unwrap(); // should be a crate there.
            state.stacks[mov.to].push_back(crat);
        }
    }

    for stack in state.stacks {
        print!("{}", stack.back().unwrap());
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{Move, parse_input};

    #[test]
    fn parse() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let parsed = parse_input(Cursor::new(input));
        println!("{parsed:?}");
        assert!(parsed.stacks[0].eq(&['Z', 'N']));
        assert!(parsed.stacks[1].eq(&['M', 'C', 'D']));
        assert!(parsed.stacks[2].eq(&['P']));
        assert_eq!(
            parsed.moves[1],
            Move {
                count: 3,
                from: 0,
                to: 2,
            }
        )
    }

    #[test]
    fn parse2() {
        let input = r#"[P]     [L]         [T]            
[L]     [M] [G]     [G]     [S]    
[M]     [Q] [W]     [H] [R] [G]    
[N]     [F] [M]     [D] [V] [R] [N]
[W]     [G] [Q] [P] [J] [F] [M] [C]
[V] [H] [B] [F] [H] [M] [B] [H] [B]
[B] [Q] [D] [T] [T] [B] [N] [L] [D]
[H] [M] [N] [Z] [M] [C] [M] [P] [P]
 1   2   3   4   5   6   7   8   9 
        
        move 8 from 3 to 2"#;
        let parsed = parse_input(Cursor::new(input));
        println!("{parsed:?}");
        assert!(parsed.stacks[7].eq(&['P', 'L', 'H', 'M', 'R', 'G', 'S']));
    }
}
