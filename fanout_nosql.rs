// Combination #11 — FAN-OUT × NOSQL (CWE-943, Rust).
pub fn fanout_nosql(u: &str) {
    let _ = format!("a='{}'", u);
    let _ = format!("b='{}'", u);
}
