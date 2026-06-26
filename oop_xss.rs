// c05 OOP × xss (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { println!("<p>{{}}</p>", self.v); }
}
pub fn oop_xss(q: String) {
    Holder { v: q }.run();
}
