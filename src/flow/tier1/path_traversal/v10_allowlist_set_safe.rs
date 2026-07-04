// v10 allowlist-set
use std::collections::HashSet;
pub fn read(target: &str) -> bool {
    ["/var/log/app.log"].into_iter().collect::<HashSet<&str>>().contains(target)
}
