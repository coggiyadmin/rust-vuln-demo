pub fn sk_framework_native_pathtrav(p: &str) {
    let safe = p.replace('<', "&lt;");
    let _ = format!("/data/{}", safe);
}
