// TN — benign output rendering; fixed app-authored result, no model output.
#![allow(dead_code)]
pub fn render_summary(count: usize, total: f64) -> String {
    format!("Processed {} items totalling {:.2}.", count, total)
}
