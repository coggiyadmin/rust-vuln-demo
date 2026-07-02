pub fn sk_hardening_ldap(uid: &str) {
    let safe = uid.replace('*', "");
    let _ = format!("(uid={})", safe);
}
