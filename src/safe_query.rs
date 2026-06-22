//! SAFE / TN fixture — database access via bound parameters only.
//! A minimal parameterized-client abstraction stands in for a real driver
//! (sqlx/postgres). User values are always passed as bound params, never
//! concatenated into SQL. The scanner MUST produce ZERO security findings;
//! any sql_injection finding is a FALSE POSITIVE.

/// Stand-in for a parameterized DB client: SQL is constant, values are bound.
pub trait Db {
    fn query(&self, sql: &'static str, params: &[&str]) -> Vec<String>;
}

pub struct UserStore<'a> {
    pub db: &'a dyn Db,
}

impl<'a> UserStore<'a> {
    /// Lookup by email — the SQL text is a constant; `email` is a bound param.
    pub fn find_by_email(&self, email: &str) -> Vec<String> {
        self.db
            .query("SELECT id, email FROM users WHERE email = $1", &[email])
    }

    /// Search — still a constant query, the fragment is bound.
    pub fn search(&self, fragment: &str) -> Vec<String> {
        self.db.query(
            "SELECT id, email FROM users WHERE email LIKE $1 LIMIT 50",
            &[fragment],
        )
    }
}
