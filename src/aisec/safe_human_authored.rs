//! SAFE mirror.
pub fn sum_positive(values: &[i32]) -> i32 {
    values.iter().copied().filter(|v| *v > 0).sum()
}
