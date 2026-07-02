pub fn sk_wrong_context_fmtstr(pat: &str) {
    let safe = pat.replace('<', "");
    let _ = format!("{}", safe);
}
