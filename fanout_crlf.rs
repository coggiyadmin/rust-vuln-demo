// c11 fanout × crlf (Rust parity)
pub fn fanout_crlf(u: String) {
    println!("{}", u);
    let _ = u.clone();
}
