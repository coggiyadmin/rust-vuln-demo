// v09 guard-in-variable
use std::collections::HashSet;
pub fn loads(raw: &str) -> bool {
    ["{}", "[]"].into_iter().collect::<HashSet<&str>>().contains(raw)
}
