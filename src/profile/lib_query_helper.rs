// FP-target (elarasu cognium-dev#128/#140) — LIBRARY profile. `where_clause` is caller-supplied,
// not an HTTP entry point.
#![allow(dead_code)]
pub fn by_filter(where_clause: &str) -> String {
    format!("SELECT * FROM items WHERE {}", where_clause) // caller-supplied, not entry point
}
