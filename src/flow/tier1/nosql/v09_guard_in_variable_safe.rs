// v09 guard-in-variable
use std::collections::HashSet;
pub fn find(col: &str) -> bool {
    ["name", "status"].into_iter().collect::<HashSet<&str>>().contains(col)
}
