// c02 path-sensitivity × ssrf (Rust parity)
pub fn path_sensitive_ssrf(v: String) {
    if !v.starts_with("safe") {
        let _ = v;
    }
}
