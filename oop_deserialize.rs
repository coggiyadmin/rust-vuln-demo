// c05 OOP × deserialize (Rust parity)
struct Holder { v: String }
impl Holder {
    fn run(&self) { let _ = serde_json::from_str::<serde_json::Value>(&self.v); }
}
pub fn oop_deserialize(q: String) {
    Holder { v: q }.run();
}
