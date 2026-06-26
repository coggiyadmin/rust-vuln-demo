use std::process::Command;
pub fn grep(p: &str) { let _ = Command::new("grep").arg(p).arg("/var/log/app.log").status(); }
