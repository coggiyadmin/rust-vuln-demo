// c04 async × xss (Rust parity)
pub fn async_taint_xss(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
