// c11 fanout × nosql (Rust parity)
pub fn fanout_nosql(u: String) {
    println!("{}", u);
    let _ = u.clone();
}
