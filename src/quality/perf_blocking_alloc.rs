//! Performance anti-pattern.
use std::fs;

pub fn process_log(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}
