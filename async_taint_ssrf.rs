// c04 async × ssrf (Rust parity)
pub fn async_taint_ssrf(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
