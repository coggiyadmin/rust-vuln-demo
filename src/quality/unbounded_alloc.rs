// Performance — unbounded allocation anti-pattern.
pub fn collect_all(items: &[usize]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for &n in items {
        out.push(vec![0u8; n]); // unbounded per-item alloc
    }
    out
}
