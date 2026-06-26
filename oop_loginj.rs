// c05 OOP × loginj (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { println!("user={{}}", self.v); }
}
pub fn oop_loginj(q: String) {
    Holder { v: q }.run();
}
