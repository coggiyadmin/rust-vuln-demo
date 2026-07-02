// c06/c07 sanitizer × ldap (Rust parity)
fn fake(s: String) -> String { s }
pub fn sanitizer_wrong_ldap(v: String) {
    let _ = v.replace('<', "_");
}
pub fn sanitizer_fake_ldap(v: String) {
    let _ = fake(v);
}
