pub fn sk_fake_loginj(user: &str, pw: &str) {
    let safe = user.replace("SAFE", "");
    let _ = format!("login user={} password={}", safe, pw);
}
