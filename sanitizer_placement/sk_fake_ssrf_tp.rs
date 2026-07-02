pub fn sk_fake_ssrf(url: &str) {
    let _ = format!("https://api.example.com/{}", url);
}
