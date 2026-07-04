// v10 allowlist-set — parameterized shape after id allowlist gate.
use std::collections::HashSet;

pub fn lookup(user_id: i64) -> Option<String> {
    let safe: HashSet<i64> = [1, 2, 3].into_iter().collect();
    if safe.contains(&user_id) {
        Some(format!("SELECT * FROM users WHERE id={user_id}"))
    } else {
        None
    }
}
