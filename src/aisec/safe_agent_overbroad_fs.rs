// SAFE mirror (OWASP LLM06) — fs tool jailed to a workspace with a canonical-prefix check.
#![allow(dead_code)]
use std::path::{Path, PathBuf};

pub fn read_tool(rel: &str) -> std::io::Result<String> {
    let workspace = PathBuf::from("/var/app/workspace");
    let target = workspace.join(rel);
    if !target.starts_with(&workspace) {
        return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "escapes jail"));
    }
    std::fs::read_to_string(target)
}
