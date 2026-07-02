pub fn sk_hardening_xxe(body: &str) {
    if body.len() > 65536 { return; }
    let _ = body.parse::<()>(); // XXE stub sink
}
