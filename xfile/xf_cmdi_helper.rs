//! Cross-file taint — SINK side (command injection, CWE-78). Imported by
//! xf_cmdi_controller.rs; `host` arrives tainted across the file boundary.
use std::process::Command;

pub fn run_ping(host: &str) {
    // SINK (CWE-78): tainted host concatenated into a shell command string.
    Command::new("sh")
        .arg("-c")
        .arg(format!("ping -c 1 {}", host))
        .status()
        .ok();
}
