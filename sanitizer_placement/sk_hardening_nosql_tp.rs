pub fn sk_hardening_nosql(user: &str) {
    let safe = user.replace('$', "");
    let _ = format!("{{\"user\":\"{}\"}}", safe);
}
