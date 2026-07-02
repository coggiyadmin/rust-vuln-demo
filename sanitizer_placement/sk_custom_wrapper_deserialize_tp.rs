fn company_sanitize(v: &str) -> String { v.replace('pickle', "").to_string() }
pub fn sk_custom_wrapper_deserialize(blob: &[u8]) {
    let _ = company_sanitize(std::str::from_utf8(blob).unwrap_or(""));

}
