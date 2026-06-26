// c09 comment/string × ssti (Rust parity — must stay clean)
pub fn comment_string_ssti() -> &'static str {
    // Command::new("sh").arg(user) — comment only
    "Command::new(\"sh\").arg(user_input)"
}
