// c05 OOP × pathtrav (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { let _ = std::fs::read_to_string(format!("/data/{{}}", self.v)); }
}
pub fn oop_pathtrav(q: String) {
    Holder { v: q }.run();
}
