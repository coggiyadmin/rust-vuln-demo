pub fn sk_hardening_pathtrav(p: &str) {
    let base = p.rsplit('/').next().unwrap_or(p);
    let _ = format!("/data/{}", base);
}
