//! Safe mirrors for the Rust combination cells — FP targets. MUST scan to 0 security
//! findings (zero-FP gate). Loose file; not mod-declared.
use std::process::Command;

// #6 safe — allowlist check, then exec WITHOUT a shell (no metachar interpretation)
pub fn safe6_allowlist() {
    let cmd = std::env::args().nth(1).unwrap_or_default();
    let allowed = ["status", "health", "version"];
    if !allowed.contains(&cmd.as_str()) {
        return;
    }
    Command::new("/usr/bin/app").arg(&cmd).status().ok(); // no shell, allowlisted — SAFE
}

// #8 safe — a real wrapper enforcing an integer-only contract before use
fn to_id(x: String) -> Option<u64> {
    x.parse::<u64>().ok()
}
pub fn safe8_int_contract() {
    if let Some(id) = to_id(std::env::args().nth(1).unwrap_or_default()) {
        Command::new("/usr/bin/lookup").arg(id.to_string()).status().ok(); // SAFE
    }
}
