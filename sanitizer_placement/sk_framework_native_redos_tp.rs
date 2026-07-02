pub fn sk_framework_native_redos(pat: &str, text: &str) {
    let safe = pat.replace('<', "");
    let _ = safe.matches(text);
}
