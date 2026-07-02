pub fn diesel_raw(id: &str) -> String {
    format!("SELECT * FROM users WHERE id={}", id) // SINK CWE-89 pattern
}
