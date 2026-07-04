// v09 guard-in-variable
use std::collections::HashSet;
pub fn search(base: &str) -> bool { ["dc=example,dc=com"].into_iter().collect::<HashSet<&str>>().contains(base) }
