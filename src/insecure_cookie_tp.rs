// TP (CWE-614/1004, rust) — Set-Cookie built without the Secure / HttpOnly attributes.
#![allow(dead_code)]

pub fn set_session(sid: &str) -> String {
    // SINK (CWE-614/1004): no `; Secure` and no `; HttpOnly` appended.
    format!("Set-Cookie: SESSIONID={}", sid)
}
