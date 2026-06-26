// c02 path-sensitivity × deserialize (Rust parity)
pub fn path_sensitive_deserialize(v: String) {
    if !v.starts_with("safe") {
        let _ = v;
    }
}
