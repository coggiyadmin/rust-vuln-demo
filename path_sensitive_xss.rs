// c02 path-sensitivity × xss (Rust parity)
pub fn path_sensitive_xss(v: String) {
    if !v.starts_with("safe") {
        let _ = v;
    }
}
