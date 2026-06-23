//! McCabe HIGH CC.
pub fn classify(a: i32, b: i32, c: i32, d: i32, kind: &str) -> &'static str {
    if kind == "x" {
        if a > 0 {
            if b > 0 {
                if c > 0 {
                    return if d > 0 { "xppp" } else { "xppn" };
                }
            }
        }
    }
    "default"
}
