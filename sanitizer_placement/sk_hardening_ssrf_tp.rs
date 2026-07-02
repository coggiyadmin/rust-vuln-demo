pub fn sk_hardening_ssrf(url: &str) {
    if url.contains("127.0.0.1") || url.contains("169.254.169.254") { return; }
    let _ = format!("https://api.example.com/{}", url);
}
