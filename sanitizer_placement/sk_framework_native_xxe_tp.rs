pub fn sk_framework_native_xxe(xml: &str) {
    let safe = xml.replace('<', "&lt;");
    let _ = safe.len();
}
