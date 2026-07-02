pub fn sk_hardening_fmtstr(fmt: &str) {
    let pat = if fmt.len() > 32 { "%s" } else { fmt };
    let _ = format!("{}", pat);
}
