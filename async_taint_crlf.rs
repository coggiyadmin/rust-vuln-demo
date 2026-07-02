// c04 async × crlf (Rust parity)
pub fn async_taint_crlf(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
