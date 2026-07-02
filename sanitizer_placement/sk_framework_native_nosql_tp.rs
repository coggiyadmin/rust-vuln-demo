pub fn sk_framework_native_nosql(user: &str) {
    let _ = format!("{{"user":{{"$eq":"{}"}}}}", user);
}
