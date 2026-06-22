//! Cross-file taint — SOURCE side (SQL injection, CWE-89).
//! Reads a process arg and passes it across the file boundary to the sink in
//! xf_sqli_helper.rs. Directory scan required (#106 — Rust MISS at v3.88.1).
mod xf_sqli_helper;

pub fn handle() {
    let name = std::env::args().nth(1).unwrap_or_default(); // SOURCE
    let _ = xf_sqli_helper::find_user(&name);               // → sink in helper (CWE-89)
}
