pub fn sk_custom_wrapper_openredirect(nxt: &str) {
    let safe = nxt.replace("javascript:", "");
    let _ = format!("redirect: {}", safe);
}
