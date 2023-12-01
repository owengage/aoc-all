use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn lines(path: &str) -> Vec<String> {
    let input = BufReader::new(File::open(path).unwrap());
    // Use this rather than flatten, so that we panic if there's an issue rather
    // than mask the error.
    let lines: Result<Vec<String>, _> = input.lines().collect();
    lines.unwrap()
}
