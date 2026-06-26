// c06/c07 sanitizer × xss (Rust parity)
fn fake(s: String) -> String { s }
pub fn sanitizer_wrong_xss(v: String) {
    let _ = v.replace('<', "_");
}
pub fn sanitizer_fake_xss(v: String) {
    let _ = fake(v);
}
