pub fn sk_custom_wrapper_xxe(body: &str) {
    let safe = body.replace("ENTITY", "");
    let _ = safe.parse::<()>(); // XXE stub sink
}
