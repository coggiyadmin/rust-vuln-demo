//! Combination cells missing from combos.rs: #4 thread, #6 wrong-context sanitizer,
//! #8 custom-wrapper sanitizer, #13 encoded. All FN: must fire (expected MISS today, #82).
//! Loose file (not mod-declared); scanner reads it in place.
use std::process::Command;

// #4 thread/async — taint moved into a spawned thread before the sink
pub fn c4_thread() {
    let cmd = std::env::args().nth(1).unwrap_or_default(); // SOURCE
    std::thread::spawn(move || {
        Command::new("sh").arg("-c").arg(&cmd).status().ok(); // SINK (CWE-78)
    })
    .join()
    .ok();
}

// #6 wrong-context sanitizer — HTML-escape applied before a COMMAND sink (wrong context)
pub fn c6_wrong_context() {
    let raw = std::env::args().nth(1).unwrap_or_default();
    let escaped = raw.replace('<', "&lt;").replace('>', "&gt;"); // not a shell sanitizer
    Command::new("sh").arg("-c").arg(format!("echo {}", escaped)).status().ok(); // SINK (CWE-78) must still fire
}

// #8 custom-wrapper sanitizer — wrapper only trims; does not neutralize shell metachars
fn sanitize(x: String) -> String { x.trim().to_string() }
pub fn c8_custom_wrapper() {
    let cmd = sanitize(std::env::args().nth(1).unwrap_or_default());
    Command::new("sh").arg("-c").arg(&cmd).status().ok(); // SINK (CWE-78) — wrapper must not be credited
}

// #13 encoded — hex-decode tainted input (std-only) then sink; decode must preserve taint
pub fn c13_encoded() {
    let hexed = std::env::args().nth(1).unwrap_or_default(); // SOURCE (hex-encoded)
    let bytes: Vec<u8> = hexed
        .as_bytes()
        .chunks(2)
        .filter_map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap_or("0"), 16).ok())
        .collect();
    let cmd = String::from_utf8_lossy(&bytes).to_string();
    Command::new("sh").arg("-c").arg(&cmd).status().ok(); // SINK (CWE-78)
}
