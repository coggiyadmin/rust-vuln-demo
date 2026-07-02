pub fn sk_framework_native_ldap(uid: &str) {
    let safe = uid.replace('<', "");
    let _ = format!("(uid={})", safe);
}
