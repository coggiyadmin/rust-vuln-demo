// c06/c07 sanitizer × loginj (Rust parity)
fn fake(s: String) -> String { s }
pub fn sanitizer_wrong_loginj(v: String) {
    let _ = v.replace('<', "_");
}
pub fn sanitizer_fake_loginj(v: String) {
    let _ = fake(v);
}
