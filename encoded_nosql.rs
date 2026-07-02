// Combination #13 — ENCODED × NOSQL (CWE-943, Rust).
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn enc_b64_nosql(raw: &str) {
    let u = String::from_utf8_lossy(&STANDARD.decode(raw).unwrap_or_default());
    let _ = format!("user == '{}'", u); // CWE-943
}
