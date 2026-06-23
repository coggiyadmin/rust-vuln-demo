// TN — benign similarity over a fixed single-owner list.
#![allow(dead_code)]
pub fn nearest(vec: &[f64]) -> &'static str {
    if !vec.is_empty() && vec[0] >= 0.5 { "greeting" } else { "farewell" }
}
