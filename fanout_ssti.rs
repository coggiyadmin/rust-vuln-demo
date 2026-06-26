// c11 fanout × ssti (Rust parity)
pub fn fanout_ssti(u: String) {
    println!("{}", u);
    let _ = u.clone();
}
