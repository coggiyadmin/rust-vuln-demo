// Excessive Agency (OWASP LLM06) — over-broad fs tool rooted at "/", no jail.
#![allow(dead_code)]
use std::path::Path;

pub fn read_tool(rel: &str) -> std::io::Result<String> {
    // SINK (LLM06): whole filesystem in scope, no path jail
    std::fs::read_to_string(Path::new("/").join(rel))
}
