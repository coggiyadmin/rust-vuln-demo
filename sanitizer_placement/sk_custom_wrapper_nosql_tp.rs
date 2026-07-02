fn company_sanitize(v: &str) -> String { v.replace('$', "").to_string() }
pub fn sk_custom_wrapper_nosql(user: &str) {
    let _ = format!("{{"user":"{}"}}", company_sanitize(user));
}
