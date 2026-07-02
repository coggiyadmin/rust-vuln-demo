pub fn sk_hardening_sqli(n: &str) {
    let safe = n.replace("--", "");
    let _ = format!("SELECT * FROM u WHERE n='{}'", safe);
}
