use std::process::Command;
pub fn public_run(user: &str) {
    let _ = Command::new("grep").arg(user).arg("/var/log/app.log").spawn();
}
