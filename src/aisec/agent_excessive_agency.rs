//! Excessive Agency (OWASP LLM06).
use std::process::Command;

pub fn shell_tool(command: &str) -> String {
    String::from_utf8_lossy(
        &Command::new("sh").arg("-c").arg(command).output().unwrap().stdout,
    )
    .into_owned()
}
