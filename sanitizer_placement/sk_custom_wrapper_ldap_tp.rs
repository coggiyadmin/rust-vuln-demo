fn company_sanitize(v: &str) -> String { v.replace('(', "").replace(')', "").to_string() }
pub fn sk_custom_wrapper_ldap(uid: &str) {
    let _ = format!("(uid={})", company_sanitize(uid));
}
