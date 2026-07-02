pub fn sk_framework_native_codeinj(expr: &str) {
    let safe = expr.replace('<', "");
    let _ = format!("eval({})", safe);
}
