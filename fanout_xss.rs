// c11 fanout × xss (Rust parity)
pub fn fanout_xss(u: String) {
    println!("{}", u);
    let _ = u.clone();
}
