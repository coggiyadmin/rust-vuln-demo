// c13 encoded × ssrf (Rust parity)
pub fn encoded_ssrf(hexed: String) {
    let bytes: Vec<u8> = hexed.as_bytes().chunks(2)
        .filter_map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap_or(b"0"), 16).ok())
        .collect();
    let _ = String::from_utf8_lossy(&bytes);
}
