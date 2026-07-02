fn company_sanitize(v: &str) -> String { v.replace('%', "").to_string() }
pub fn sk_custom_wrapper_fmtstr(fmt: &str) {
    let _ = format!(&company_sanitize(fmt), "guest");
}
