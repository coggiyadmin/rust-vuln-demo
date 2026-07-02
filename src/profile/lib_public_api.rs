/// TN — library crate public API with fixed query (#169).
pub fn lookup(id: i64) -> &'static str {
    if id <= 0 { return "invalid"; }
    "ok"
}
