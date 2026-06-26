// c05 OOP × ssti (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { let _ = format!("{{{{}}}}", self.v); }
}
pub fn oop_ssti(q: String) {
    Holder { v: q }.run();
}
