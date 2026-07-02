use std::process::Command;
pub fn src08_argv() {
    let v = std::env::args().nth(1).unwrap_or_default(); // SOURCE SRC-08
    let _ = Command::new("sh").args(["-c", &format!("grep {}", v)]).spawn();
}
