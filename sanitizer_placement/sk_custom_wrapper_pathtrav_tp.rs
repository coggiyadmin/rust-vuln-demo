pub fn sk_custom_wrapper_pathtrav(p: &str) {
    let safe = p.trim_start_matches('/');
    let _ = format!("/data/{}", safe);
}
