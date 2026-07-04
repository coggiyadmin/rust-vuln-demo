// v09 guard-in-variable
use std::collections::HashSet;
pub fn query(expr: &str) -> bool { ["//users/user"].into_iter().collect::<HashSet<&str>>().contains(expr) }
