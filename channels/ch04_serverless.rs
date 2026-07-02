use std::process::Command;
// ch04 serverless event → cmdi sink
pub fn serverless(q: &str) {
    let _ = Command::new("sh").args(["-c", &format!("grep {}", q)]); // SOURCE ch04
}
