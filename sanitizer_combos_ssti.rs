// c06/c07 sanitizer × ssti (Rust parity)
fn fake(s: String) -> String { s }
pub fn sanitizer_wrong_ssti(v: String) {
    let _ = v.replace('<', "_");
}
pub fn sanitizer_fake_ssti(v: String) {
    let _ = fake(v);
}
