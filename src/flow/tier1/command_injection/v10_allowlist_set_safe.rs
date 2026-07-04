// v10 allowlist-set
use std::collections::HashSet;
use std::process::Command;
pub fn run(cmd: &str, path: &str) {
    let safe: HashSet<String> = ["grep:/var/log/app.log".into()].into_iter().collect();
    if safe.contains(&format!("{cmd}:{path}")) { let _ = Command::new(cmd).arg(path).spawn(); }
}
