// c04 async × pathtrav (Rust parity)
pub fn async_taint_pathtrav(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
