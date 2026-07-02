fn company_sanitize(v: &str) -> String {
    v.replace(''', "").replace('"', "").to_string()
}
pub fn sk_custom_wrapper_xpath(name: &str) {
    let _ = format!("//user[name='{}']", company_sanitize(name));
}
