// v09 guard-in-variable
use std::collections::HashSet;
pub fn log_level(level: &str) -> bool { ["INFO", "WARN"].into_iter().collect::<HashSet<&str>>().contains(level) }
