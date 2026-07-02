// c05 OOP × xxe (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { let _ = self.v.parse::<()>(); }
}
pub fn oop_xxe(q: String) {
    Holder { v: q }.run();
}
