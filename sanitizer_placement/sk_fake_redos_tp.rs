pub fn sk_fake_redos(pattern: &str, text: &str) {
    let safe = pattern.replace("SAFE", "");
    let _ = safe.matches(text);
}
