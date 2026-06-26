// c03 loop × pathtrav (Rust parity)
pub fn loop_taint_pathtrav(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
