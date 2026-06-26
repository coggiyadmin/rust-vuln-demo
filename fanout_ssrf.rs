// c11 fanout × ssrf (Rust parity)
pub fn fanout_ssrf(u: String) {
    println!("{}", u);
    let _ = u.clone();
}
