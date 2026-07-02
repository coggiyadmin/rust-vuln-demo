pub fn sk_wrong_context_ssti(n: &str) {
    let safe = n.replace(''', "''");
    let _ = format!("<p>{}</p>", safe);
}
