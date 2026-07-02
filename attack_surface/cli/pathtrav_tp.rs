use std::process::Command;
pub fn main() {
    let v = std::env::args().nth(1).unwrap_or_default();
    let _ = Command::new("grep").arg(&v).arg("/var/log/app.log").spawn();
}
