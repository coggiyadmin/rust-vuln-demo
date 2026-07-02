pub fn sk_framework_native_ssti(uid: &str) {
    let safe = uid.replace('<', "&lt;");
    let _ = format!("SELECT * FROM u WHERE n='{}'", safe);
}
