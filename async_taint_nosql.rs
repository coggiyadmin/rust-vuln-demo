// c04 async × nosql (Rust parity)
pub fn async_taint_nosql(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
