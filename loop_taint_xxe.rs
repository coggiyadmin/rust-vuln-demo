// c03 loop × xxe (Rust parity)
pub fn loop_taint_xxe(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
