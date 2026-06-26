// c03 loop × ssrf (Rust parity)
pub fn loop_taint_ssrf(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
