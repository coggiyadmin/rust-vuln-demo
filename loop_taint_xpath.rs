// c03 loop × xpath (Rust parity)
pub fn loop_taint_xpath(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
