// c03 loop × loginj (Rust parity)
pub fn loop_taint_loginj(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
