// c08 SAFE — validated key × NoSQL. Expect clean.
pub fn wrapped_nosql(k: &str) -> bool {
    k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}
