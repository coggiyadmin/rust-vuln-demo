pub fn sk_wrong_context_ldap(uid: &str) {
    let safe = uid.replace('<', "");
    let _ = format!("(uid={})", safe);
}
