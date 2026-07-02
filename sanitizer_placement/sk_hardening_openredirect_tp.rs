pub fn sk_hardening_openredirect(nxt: &str) {
    let url = if nxt.starts_with("http://") || nxt.starts_with("https://") {
        nxt.to_string()
    } else {
        format!("https://example.com/{}", nxt)
    };
    let _ = format!("Location: {}", url);
}
