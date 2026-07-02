// c05 OOP × xpath (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { let _ = format!("//user[@id='{{}}']", self.v); }
}
pub fn oop_xpath(q: String) {
    Holder { v: q }.run();
}
