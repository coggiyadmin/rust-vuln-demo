// v09 guard-in-variable — column allowlist in HashSet.
use std::collections::HashSet;

pub fn lookup(col: &str, val: &str) -> Option<String> {
    let allowed: HashSet<&str> = ["name", "email"].into_iter().collect();
    if allowed.contains(col) {
        Some(format!("SELECT {col} FROM users WHERE {col}='{val}'"))
    } else {
        None
    }
}
