// c06/c07 sanitizer × deserialize (Rust parity)
fn fake(s: String) -> String { s }
pub fn sanitizer_wrong_deserialize(v: String) {
    let _ = v.replace('<', "_");
}
pub fn sanitizer_fake_deserialize(v: String) {
    let _ = fake(v);
}
