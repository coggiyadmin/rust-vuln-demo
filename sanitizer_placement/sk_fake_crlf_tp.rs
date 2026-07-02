pub fn sk_fake_crlf(loc: &str) {
    let safe = loc.replace("SAFE", "");
    let _ = format!("Location: {}", safe);
}
