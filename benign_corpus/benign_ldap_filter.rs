pub fn filter(uid: &str) -> bool { uid.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') }
