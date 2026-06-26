// c09 comment/string × xss (Rust parity — must stay clean)
pub fn comment_string_xss() -> &'static str {
    // Command::new("sh").arg(user) — comment only
    "Command::new(\"sh\").arg(user_input)"
}
