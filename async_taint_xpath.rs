// c04 async × xpath (Rust parity)
pub fn async_taint_xpath(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
