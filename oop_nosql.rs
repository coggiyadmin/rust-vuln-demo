// c05 OOP × nosql (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { let _ = format!("{{\"user\":\"{{}}\"}}", self.v); }
}
pub fn oop_nosql(q: String) {
    Holder { v: q }.run();
}
