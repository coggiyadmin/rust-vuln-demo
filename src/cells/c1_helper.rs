//! #1 cross-file taint — HELPER half. Sink lives here; tainted value arrives as a
//! parameter from c1.rs. Scan the directory (runbook §2). Loose file: not mod-declared,
//! so `cargo build` ignores it; the scanner reads it in place.
use std::process::Command;

pub fn run_host(host: &str) {
    // SINK (CWE-78) — taint must arrive cross-file from c1.rs
    Command::new("sh").arg("-c").arg(format!("ping -c 1 {}", host)).status().ok();
}
