use std::{
    fs::File,
    io::{BufRead, BufReader},
    mem,
};

pub mod two;

pub fn lines(path: &str) -> Vec<String> {
    let input = BufReader::new(File::open(path).unwrap());
    // Use this rather than flatten, so that we panic if there's an issue rather
    // than mask the error.
    let lines: Result<Vec<String>, _> = input.lines().collect();
    lines.unwrap()
}

/// Parse input as blocks of lines, each block should be separated be a blank line.
pub fn line_blocks(path: &str) -> Vec<Vec<String>> {
    let input = lines(path);
    let mut blocks = vec![];
    let mut current = vec![];

    for line in input {
        if line.is_empty() {
            let block = mem::take(&mut current);
            blocks.push(block);
        } else {
            current.push(line);
        }
    }
    if !current.is_empty() {
        blocks.push(current);
    }

    blocks
}

pub fn lines_from_str(input: &str) -> Vec<String> {
    let input = BufReader::new(input.as_bytes());
    // Use this rather than flatten, so that we panic if there's an issue rather
    // than mask the error.
    let lines: Result<Vec<String>, _> = input.lines().collect();
    lines.unwrap()
}
