pub fn sk_wrong_context_loginj(user: &str, pw: &str) {
    let safe = user.replace('<', "");
    let _ = format!("login user={} password={}", safe, pw);
}
