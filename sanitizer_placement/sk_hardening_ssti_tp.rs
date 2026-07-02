pub fn sk_hardening_ssti(tpl: &str) {
    let n = if tpl.len() > 64 { &tpl[..64] } else { tpl };
    let _ = format!("<p>Hello {}</p>", n);
}
