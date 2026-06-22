//! Cross-file taint — SINK side (path traversal, CWE-22). Imported by
//! xf_pathtrav_controller.rs; `name` arrives tainted across the file boundary.
use std::fs;
use std::io;

pub fn read_doc(name: &str) -> io::Result<String> {
    // SINK (CWE-22): tainted name joined to a base path with no traversal guard.
    fs::read_to_string(format!("/var/app/data/{}", name))
}
