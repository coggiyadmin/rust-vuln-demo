//! SAFE mirror.
pub fn classify(a: i32, b: i32, kind: &str) -> &'static str {
    if kind == "x" {
        return if a > 0 { "xp" } else { "xn" };
    }
    "default"
}
