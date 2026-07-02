fn company_sanitize(v: &str) -> String { v.replace('\r', "").to_string() }
pub fn sk_custom_wrapper_crlf(nxt: &str) {
    let _ = format!("Location: /home?u={}", company_sanitize(nxt));

}
