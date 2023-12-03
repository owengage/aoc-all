use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

pub mod two;

pub fn lines(path: &str) -> Vec<String> {
    let input = BufReader::new(File::open(path).unwrap());
    // Use this rather than flatten, so that we panic if there's an issue rather
    // than mask the error.
    let lines: Result<Vec<String>, _> = input.lines().collect();
    lines.unwrap()
}

pub fn lines_from_str(input: &str) -> Vec<String> {
    let input = BufReader::new(input.as_bytes());
    // Use this rather than flatten, so that we panic if there's an issue rather
    // than mask the error.
    let lines: Result<Vec<String>, _> = input.lines().collect();
    lines.unwrap()
}
