pub fn sk_fake_ssti(n: &str) {
    let _ = n.replace("SAFE", "");
    let _ = format!("<p>{}</p>", n);
}
