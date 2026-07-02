fn company_sanitize(v: &str) -> String { v.replace('SAFE', "").to_string() }
pub fn sk_custom_wrapper_redos(pat: &str, text: &str) {
    let safe = company_sanitize(pat);
    let _ = safe.matches(text);

}
