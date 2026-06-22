//! #1 cross-file taint — CALLER half. Source bound to a local, passed across the file
//! boundary to the sink in c1_helper.rs. FN: must fire (expected MISS today, #82 Rust
//! taint non-functional + cross-file #106).
mod c1_helper;

pub fn c1_crossfile() {
    let host = std::env::args().nth(1).unwrap_or_default(); // SOURCE
    c1_helper::run_host(&host);                             // crosses file boundary → SINK (CWE-78)
}
