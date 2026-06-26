// c09 comment/string × ssrf (Rust parity — must stay clean)
pub fn comment_string_ssrf() -> &'static str {
    // Command::new("sh").arg(user) — comment only
    "Command::new(\"sh\").arg(user_input)"
}
