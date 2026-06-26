// c05 OOP × openredirect (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { println!("redirect: {{}}", self.v); }
}
pub fn oop_openredirect(q: String) {
    Holder { v: q }.run();
}
