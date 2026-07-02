// c02 path-sensitivity × xxe (Rust parity)
pub fn path_sensitive_xxe(v: String) {
    if !v.starts_with("safe") {
        let _ = v;
    }
}
