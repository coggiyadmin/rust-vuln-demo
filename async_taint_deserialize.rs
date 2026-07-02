// c04 async × deserialize (Rust parity)
pub fn async_taint_deserialize(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
