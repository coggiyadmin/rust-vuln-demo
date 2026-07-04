// v09 guard-in-variable
use std::collections::HashSet;
use std::process::Command;
pub fn run(bin: &str, arg: &str) {
    let ok: HashSet<&str> = ["grep", "cat"].into_iter().collect();
    if ok.contains(bin) { let _ = Command::new(bin).arg(arg).spawn(); }
}
