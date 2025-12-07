use std::{collections::HashSet, fs::File, io::Read};

fn all_unique(window: &[u8]) -> bool {
    let s: HashSet<u8> = window.iter().copied().collect();
    s.len() == window.len()
}

fn main() {
    let mut buf = vec![];
    File::open("input.txt")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    let part1 = buf
        .windows(4)
        .enumerate()
        .find(|(_, w)| all_unique(w))
        .unwrap();

    dbg!(part1.0 + 4);

    let part2 = buf
        .windows(14)
        .enumerate()
        .find(|(_, w)| all_unique(w))
        .unwrap();

    dbg!(part2.0 + 14);
}
