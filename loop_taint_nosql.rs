// c03 loop × nosql (Rust parity)
pub fn loop_taint_nosql(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
