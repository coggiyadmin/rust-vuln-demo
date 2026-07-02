pub fn sk_hardening_deserialize(body: &[u8]) {
    if body.len() > 65536 { return; }
    let _ = body; // deser stub sink
}
