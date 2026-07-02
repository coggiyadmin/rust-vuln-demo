pub fn sk_hardening_loginj(user: &str, pw: &str) {
    let u = user.replace('
', "");
    let p = pw.replace('
', "");
    let _ = format!("login user={} password={}", u, p);
}
