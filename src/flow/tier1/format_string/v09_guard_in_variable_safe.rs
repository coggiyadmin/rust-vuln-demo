// v09 guard-in-variable
use std::collections::HashSet;
pub fn fmt(t: &str) -> bool { ["user=%s"].into_iter().collect::<HashSet<&str>>().contains(t) }
