// c05 OOP × crlf (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { println!("Location: {{}}", self.v); }
}
pub fn oop_crlf(q: String) {
    Holder { v: q }.run();
}
