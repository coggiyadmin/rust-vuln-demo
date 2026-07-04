// v09 guard-in-variable
use std::collections::HashSet;
pub fn parse(raw: &str) -> bool { ["<root/>"].into_iter().collect::<HashSet<&str>>().contains(raw) }
