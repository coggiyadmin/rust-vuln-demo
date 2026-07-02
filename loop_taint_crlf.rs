// c03 loop × crlf (Rust parity)
pub fn loop_taint_crlf(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
