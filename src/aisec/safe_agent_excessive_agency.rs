//! SAFE mirror.
use std::process::Command;

pub fn shell_tool(command: &str) -> Result<String, String> {
    if command != "pwd" && command != "date" {
        return Err("not allowed".into());
    }
    Ok(String::from_utf8_lossy(&Command::new(command).output().unwrap().stdout).into_owned())
}
