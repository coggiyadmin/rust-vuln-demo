pub fn sk_framework_native_sqli(n: &str) {
    let _ = format!("SELECT * FROM u WHERE n='{}'", n);
}
