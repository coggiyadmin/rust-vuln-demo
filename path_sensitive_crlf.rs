// c02 path-sensitivity × crlf (Rust parity)
pub fn path_sensitive_crlf(v: String) {
    if !v.starts_with("safe") {
        let _ = v;
    }
}
