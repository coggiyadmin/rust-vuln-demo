pub fn sk_framework_native_ssrf(url: &str) {
    let safe = url.replace('<', "");
    let _ = format!("fetch:{}", safe);
}
