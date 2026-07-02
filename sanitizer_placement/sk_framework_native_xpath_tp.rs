pub fn sk_framework_native_xpath(q: &str) {
    let safe = q.replace('<', "");
    let _ = format!("//u[@n='{}']", safe);
}
