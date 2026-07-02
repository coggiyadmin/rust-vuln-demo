// Combinations #6/#7 — SANITIZER × NOSQL (CWE-943, Rust).
pub fn wrong_ctx(user: &str) {
    let safe = user.replace('<', "_");
    let _ = format!("$where: this.a == '{}'", safe); // CWE-943
}
