// c03 loop × openredirect (Rust parity)
pub fn loop_taint_openredirect(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
