pub fn sk_wrong_context_xpath(name: &str) {
    let safe = name.replace('<', "");
    let _ = format!("//user[name='{}']", safe);
}
