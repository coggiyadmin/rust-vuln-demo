// c04 async × loginj (Rust parity)
pub fn async_taint_loginj(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
