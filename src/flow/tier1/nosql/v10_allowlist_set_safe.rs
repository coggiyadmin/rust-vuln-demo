// v10 allowlist-set
use std::collections::HashSet;
pub fn lookup(id: i64) -> bool { [1, 2, 3].into_iter().collect::<HashSet<i64>>().contains(&id) }
