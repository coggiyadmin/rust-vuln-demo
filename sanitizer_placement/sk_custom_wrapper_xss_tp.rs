fn company_sanitize(v: &str) -> String {
    v.replace('<', "").replace('>', "").to_string()
}
pub fn sk_custom_wrapper_xss(q: &str) {
    let _ = format!("<p>{}</p>", company_sanitize(q));
}
