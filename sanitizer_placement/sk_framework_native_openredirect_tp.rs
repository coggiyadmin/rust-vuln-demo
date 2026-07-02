pub fn sk_framework_native_openredirect(nxt: &str) {
    let safe = nxt.replace('<', "&lt;");
    let _ = format!("Location: {}", safe);
}
