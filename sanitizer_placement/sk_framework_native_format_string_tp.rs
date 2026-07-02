pub fn sk_framework_native_fmtstr(fmt: &str, name: &str) {
    let safe = name.replace('<', "&lt;");
    let _ = format!("{}", fmt.replace("{}", &safe));
}
