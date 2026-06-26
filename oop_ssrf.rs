// c05 OOP × ssrf (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { let _ = reqwest::blocking::get(&self.v); }
}
pub fn oop_ssrf(q: String) {
    Holder { v: q }.run();
}
