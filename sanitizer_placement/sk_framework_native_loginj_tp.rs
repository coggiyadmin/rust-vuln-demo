pub fn sk_framework_native_loginj(user: &str) {
    let safe = user.replace('<', "&lt;");
    println!("login user={}", safe);
}
