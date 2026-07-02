pub fn sk_hardening_crlf(loc: &str) {
    let safe = loc.replace('
', "");
    let _ = format!("Location: {}", safe);
}
