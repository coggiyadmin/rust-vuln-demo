pub fn sk_fake_fmtstr(pat: &str) {
    let safe = pat.replace("SAFE", "");
    let _ = format!("{}", safe);
}
