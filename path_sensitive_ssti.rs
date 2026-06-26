// c02 path-sensitivity × ssti (Rust parity)
pub fn path_sensitive_ssti(v: String) {
    if !v.starts_with("safe") {
        let _ = v;
    }
}
