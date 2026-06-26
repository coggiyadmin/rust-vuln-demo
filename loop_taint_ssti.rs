// c03 loop × ssti (Rust parity)
pub fn loop_taint_ssti(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
