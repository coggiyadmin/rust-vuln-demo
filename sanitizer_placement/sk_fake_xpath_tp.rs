pub fn sk_fake_xpath(name: &str) {
    let safe = name.replace("SAFE", "");
    let _ = format!("//user[name='{}']", safe);
}
