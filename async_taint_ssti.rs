// c04 async × ssti (Rust parity)
pub fn async_taint_ssti(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
