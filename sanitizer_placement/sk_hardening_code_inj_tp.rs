pub fn sk_hardening_codeinj(expr: &str) {
    let safe = expr.replace("import", "");
    let _ = format!("eval({})", safe);
}
