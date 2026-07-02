pub fn sk_hardening_redos(pattern: &str, text: &str) {
    if pattern.len() > 64 { return; }
    let _ = pattern.matches(text);
}
