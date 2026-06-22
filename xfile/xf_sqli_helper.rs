//! Cross-file taint — SINK side (SQL injection, CWE-89). Imported by
//! xf_sqli_controller.rs; `name` arrives tainted across the file boundary.

/// Minimal stand-in DB client (avoids an external crate dependency).
fn db_execute(sql: &str) -> Vec<String> {
    let _ = sql;
    Vec::new()
}

pub fn find_user(name: &str) -> Vec<String> {
    // SINK (CWE-89): tainted name concatenated directly into the SQL string.
    let sql = format!("SELECT * FROM users WHERE name = '{}'", name);
    db_execute(&sql)
}
