fn company_sanitize(v: &str) -> String { v.replace(';', "").to_string() }
pub fn sk_custom_wrapper_cmdi(q: &str) {
    let _ = format!("grep {}", company_sanitize(q));
}
