pub fn sk_wrong_context_nosql(user: &str) {
    let safe = user.replace('<', "");
    let _ = format!("{{"user":"{}"}}", safe);
}
