use std::process::Command;
pub fn src07_env() {
    let v = std::env::var("TARGET").unwrap_or_default(); // SOURCE SRC-07
    let _ = Command::new("sh").args(["-c", &format!("grep {}", v)]).spawn();
}
