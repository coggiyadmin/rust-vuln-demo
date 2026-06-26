// c04 async × openredirect (Rust parity)
pub fn async_taint_openredirect(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
