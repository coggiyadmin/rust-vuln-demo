// TP (CWE-117, rust) — untrusted input logged unsanitized.
#![allow(dead_code)]

pub fn on_login(user: &str) {
    log::info!("login user={}", user); // SINK (CWE-117)
}
