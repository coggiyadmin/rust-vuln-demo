//! Cross-file taint — SOURCE side (path traversal, CWE-22).
//! Reads a process arg and passes it across the file boundary to the sink in
//! xf_pathtrav_helper.rs. Directory scan required (#106 — Rust MISS at v3.88.1).
mod xf_pathtrav_helper;

pub fn handle() {
    let name = std::env::args().nth(1).unwrap_or_default(); // SOURCE
    let _ = xf_pathtrav_helper::read_doc(&name);            // → sink in helper (CWE-22)
}
