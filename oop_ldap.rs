// c05 OOP × ldap (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { let _ = format!("(uid={{}})", self.v); }
}
pub fn oop_ldap(q: String) {
    Holder { v: q }.run();
}
