//! Cross-file taint — SOURCE side (command injection, CWE-78).
//! Reads a process arg and passes it across the file boundary to the sink in
//! xf_cmdi_helper.rs. The scanner MUST trace taint across files on a DIRECTORY
//! scan; no finding = FALSE NEGATIVE (cross-file, #106 — Rust still MISS at v3.88.1).
mod xf_cmdi_helper;

pub fn handle() {
    let host = std::env::args().nth(1).unwrap_or_default(); // SOURCE
    xf_cmdi_helper::run_ping(&host);                        // → sink in helper (CWE-78)
}
