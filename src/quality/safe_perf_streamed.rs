//! SAFE mirror.
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process_log(path: &str, max: usize) -> Vec<String> {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().take(max).map(|l| l.unwrap()).collect()
}
