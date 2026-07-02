// c03 loop × ldap (Rust parity)
pub fn loop_taint_ldap(q: String) {
    let mut acc = String::new();
    for ch in q.chars() { acc.push(ch); }
    let _ = acc; // loop-carried
}
