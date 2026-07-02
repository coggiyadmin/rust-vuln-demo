fn company_sanitize(v: &str) -> String { v.replace('\n', "").to_string() }
pub fn sk_custom_wrapper_loginj(user: &str) {
    let _ = format!("login user={}", company_sanitize(user));

}
