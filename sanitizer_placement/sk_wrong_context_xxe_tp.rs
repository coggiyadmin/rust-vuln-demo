pub fn sk_wrong_context_xxe(body: &str) {
    let _ = body.replace(''', "''");
    let _ = body.parse::<()>(); // XXE stub sink
}
