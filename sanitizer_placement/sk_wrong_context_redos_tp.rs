pub fn sk_wrong_context_redos(pattern: &str, text: &str) {
    let safe = pattern.replace(''', "''");
    let _ = safe.matches(text);
}
