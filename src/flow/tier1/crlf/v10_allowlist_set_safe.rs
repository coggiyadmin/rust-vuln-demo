// v09 guard-in-variable
use std::collections::HashSet;
pub fn set_ct(v: &str) -> bool { ["text/plain"].into_iter().collect::<HashSet<&str>>().contains(v) }
