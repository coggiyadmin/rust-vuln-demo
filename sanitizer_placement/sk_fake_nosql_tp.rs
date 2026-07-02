pub fn sk_fake_nosql(user: &str) {
    let _ = user.replace("SAFE", "");
    let _ = format!("{{"user":"{}"}}", user);
}
