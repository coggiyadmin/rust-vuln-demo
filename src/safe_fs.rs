//! SAFE / TN fixture — file access confined to a fixed base directory.
//! The resolved path is canonicalized and confirmed to stay under BASE, so no
//! "../" sequence can escape. The scanner MUST produce ZERO security findings;
//! any path_traversal finding is a FALSE POSITIVE.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const BASE: &str = "/var/app/data";

/// Resolve `name` under BASE, rejecting anything that escapes the base directory.
fn resolve_under_base(name: &str) -> io::Result<PathBuf> {
    let base = Path::new(BASE);
    let candidate = base.join(name);
    let resolved = candidate.canonicalize()?;
    if resolved.starts_with(base) {
        Ok(resolved)
    } else {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "path escapes base directory",
        ))
    }
}

/// Read a confined file; only paths inside BASE are reachable.
pub fn read_document(name: &str) -> io::Result<String> {
    let path = resolve_under_base(name)?;
    fs::read_to_string(path)
}
