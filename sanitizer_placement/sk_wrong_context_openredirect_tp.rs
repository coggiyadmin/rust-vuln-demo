pub fn sk_wrong_context_openredirect(nxt: &str) {
    let safe = nxt.replace('<', "");
    let _ = format!("Location: {}", safe);
}
