// ch06 Gin-style redirect
pub fn gin_redirect(next: &str) -> String {
    format!("Location: {}", next) // SINK CWE-601
}
