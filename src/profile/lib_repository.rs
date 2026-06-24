// FP-target (upstream cognium-dev#128/#140) — DEEP real-DB flow. A repository helper builds SQL
// from a caller-supplied `filter` and executes it on a real rusqlite connection sink. `filter`
// is library-API input from the embedding application (its own config/DSL), NOT an HTTP entry
// point, so under an entry-point gate this must NOT be sql_injection — even though a genuine
// format!→execute sink is present.
#![allow(dead_code)]
use rusqlite::Connection;

/// Library data-access helper embedded by applications.
pub struct ItemRepo {
    conn: Connection,
}

impl ItemRepo {
    /// `filter` originates from the embedding app's own config, not from request data.
    pub fn find_where(&self, filter: &str) -> rusqlite::Result<usize> {
        let sql = format!("SELECT id, name FROM items WHERE {}", filter); // library-supplied operand
        self.conn.execute(&sql, []) // real sink
    }
}
