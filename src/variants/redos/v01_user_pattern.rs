pub fn redos_match(pattern: &str, text: &str) {
    let _ = pattern.matches(text); // SINK CWE-1333 (user-controlled pattern)
}
