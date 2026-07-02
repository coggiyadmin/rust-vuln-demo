pub fn sk_framework_native_cmdi(cmd: &str) {
    let safe = cmd.replace('<', "");
    let _ = format!("sh -c {}", safe);
}
