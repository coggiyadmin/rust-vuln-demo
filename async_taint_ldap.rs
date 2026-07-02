// c04 async × ldap (Rust parity)
pub fn async_taint_ldap(q: String) {
    std::thread::spawn(move || {
        let _ = q;
    }).join().ok();
}
