// v09 guard-in-variable
use std::collections::HashSet;
use std::path::PathBuf;
pub fn read(base: &str, name: &str) -> Option<PathBuf> {
    let ok: HashSet<&str> = ["/var/log", "/tmp/safe"].into_iter().collect();
    if ok.contains(base) { Some(PathBuf::from(base).join(name)) } else { None }
}
