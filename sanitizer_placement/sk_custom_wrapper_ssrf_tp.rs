pub fn sk_custom_wrapper_ssrf(url: &str) {
    let safe = url.replace("http://", "").replace("https://", "");
    let _ = format!("http://{}", safe);
}
