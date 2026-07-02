// c03 loop × deserialize (Rust parity)
pub fn loop_taint_deserialize(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
