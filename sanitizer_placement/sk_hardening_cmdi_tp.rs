pub fn sk_hardening_cmdi(cmd: &str) {
    let safe = cmd.replace(';', "");
    let _ = format!("sh -c {}", safe);
}
