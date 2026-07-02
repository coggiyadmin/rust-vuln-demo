pub fn sk_wrong_context_ssrf(url: &str) {
    let u = url.replace('<', "");
    let _ = format!("https://api.example.com/{}", u);
}
