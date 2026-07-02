pub fn sk_fake_xxe(body: &str) {
    let _ = body.replace("SAFE", "");
    let _ = body.parse::<()>(); // XXE stub sink
}
