pub fn sk_framework_native_crlf(nxt: &str) {
    let safe = nxt.replace('<', "&lt;");
    println!("Location: /home?u={}", safe);
}
