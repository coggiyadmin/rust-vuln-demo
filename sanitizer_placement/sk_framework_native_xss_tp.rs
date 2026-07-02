pub fn sk_framework_native_xss(q: &str) {
    let _ = format!("<p>{}</p>", q);
}
