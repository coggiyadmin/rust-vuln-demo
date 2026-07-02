pub fn sk_hardening_xpath(name: &str) {
    if name.len() > 256 { return; }
    let _ = format!("//user[name='{}']", name);
}
