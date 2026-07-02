pub fn sk_hardening_xss(q: &str) {
    let safe = q.replace("<script", "");
    let _ = format!("<p>{}</p>", safe);
}
