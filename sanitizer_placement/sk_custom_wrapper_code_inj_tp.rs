pub fn sk_custom_wrapper_code_inj(code: &str) {
    let safe = code.replace(';', "");
    let _ = safe;
}
