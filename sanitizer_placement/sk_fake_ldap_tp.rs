pub fn sk_fake_ldap(uid: &str) {
    let safe = uid.replace("SAFE", "");
    let _ = format!("(uid={})", safe);
}
