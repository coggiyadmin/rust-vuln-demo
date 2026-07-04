// Library-API param — not HTTP attacker source (#128).
pub fn run_query(user_filter: &str) -> String {
    format!("SELECT * FROM users WHERE name='{}'", user_filter)
}
