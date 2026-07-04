// SAFE — capped allocation.
const MAX: usize = 4096;
pub fn collect_all(items: &[usize]) -> Vec<Vec<u8>> {
    items.iter().take(100).map(|&n| vec![0u8; n.min(MAX)]).collect()
}
