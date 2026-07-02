pub fn sk_wrong_context_crlf(loc: &str) {
    let safe = loc.replace(''', "''");
    let _ = format!("Location: {}", safe);
}
