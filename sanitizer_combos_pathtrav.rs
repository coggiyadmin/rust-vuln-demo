// c06/c07 sanitizer × pathtrav (Rust parity)
fn fake(s: String) -> String { s }
pub fn sanitizer_wrong_pathtrav(v: String) {
    let _ = v.replace('<', "_");
}
pub fn sanitizer_fake_pathtrav(v: String) {
    let _ = fake(v);
}
