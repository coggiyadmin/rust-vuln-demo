// c06/c07 sanitizer × ssrf (Rust parity)
fn fake(s: String) -> String { s }
pub fn sanitizer_wrong_ssrf(v: String) {
    let _ = v.replace('<', "_");
}
pub fn sanitizer_fake_ssrf(v: String) {
    let _ = fake(v);
}
