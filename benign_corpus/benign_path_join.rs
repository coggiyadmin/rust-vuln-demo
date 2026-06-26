use std::path::{Path, PathBuf};
pub fn read_under(root: &Path, name: &str) -> std::io::Result<String> {
    let full = root.join(name).canonicalize()?;
    if !full.starts_with(root) { return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "escape")); }
    std::fs::read_to_string(full)
}
