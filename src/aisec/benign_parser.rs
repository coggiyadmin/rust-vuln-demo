// TN — benign parser; trims and drops blank lines.
#![allow(dead_code)]

pub fn parse_lines(lines: &[String]) -> Vec<String> {
    lines.iter().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect()
}
