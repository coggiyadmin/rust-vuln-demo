// c04 async × xxe (Rust parity)
pub fn async_taint_xxe(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
