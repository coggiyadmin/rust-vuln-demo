// c06/c07 sanitizer × crlf (Rust parity)
fn fake(s: String) -> String { s }
pub fn sanitizer_wrong_crlf(v: String) {
    let _ = v.replace('<', "_");
}
pub fn sanitizer_fake_crlf(v: String) {
    let _ = fake(v);
}
