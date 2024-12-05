use core::panic;
use std::{
    env,
    fs::{self, create_dir_all, File},
    io::{BufRead, BufReader, Read},
    mem,
    path::{Path, PathBuf},
};

use reqwest::Method;

pub mod two;

pub fn text(path: impl AsRef<Path>) -> String {
    let mut input = BufReader::new(File::open(path).unwrap());
    let mut ret = String::new();
    input.read_to_string(&mut ret).unwrap();
    ret
}

/// Simply get each line of input as a vector of strings.
pub fn lines(path: impl AsRef<Path>) -> Vec<String> {
    let input = BufReader::new(File::open(path).unwrap());
    // Use this rather than flatten, so that we panic if there's an issue rather
    // than mask the error.
    let lines: Result<Vec<String>, _> = input.lines().collect();
    lines.unwrap()
}

/// Parse input as blocks of lines, each block should be separated be a blank line.
pub fn line_blocks(path: impl AsRef<Path>) -> Vec<Vec<String>> {
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

/// Same as lines but for string input, useful for tests.
pub fn lines_from_str(input: &str) -> Vec<String> {
    let input = BufReader::new(input.as_bytes());
    // Use this rather than flatten, so that we panic if there's an issue rather
    // than mask the error.
    let lines: Result<Vec<String>, _> = input.lines().collect();
    lines.unwrap()
}

pub trait StrExt {
    fn strip_brackets(&self, left: char, right: char) -> Option<Self>
    where
        Self: Sized;
}

impl StrExt for &str {
    fn strip_brackets(&self, left: char, right: char) -> Option<Self> {
        let s = self.strip_prefix(left)?;
        s.strip_suffix(right)
    }
}

/// Get input for the given day using API key. Caches results. Panics on
/// basically any issue.
pub fn fetch_input(year: usize, day: usize) -> PathBuf {
    let input_dir = env::var("AOC_INPUT_DIR").unwrap();
    let save_path = format!("{input_dir}/{year}/day{day}").into();

    if fs::exists(&save_path).unwrap() {
        return save_path;
    }

    let _ = create_dir_all(format!("{input_dir}/{year}"));

    let api_key = env::var("AOC_KEY").unwrap();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = reqwest::blocking::Client::new();
    let resp = client
        .request(Method::GET, url)
        .header("Cookie", format!("session={}", api_key.trim()))
        .send()
        .unwrap();

    if resp.status().is_success() {
        fs::write(&save_path, resp.text().unwrap()).unwrap();
        save_path
    } else {
        println!("{:?}", resp.text());
        panic!("failed to get input");
    }
}
