// Provenance FN (OWASP LLM09) — laundered AI authorship; template-shaped. MISS.
#![allow(dead_code)]

pub fn compute_weighted_average(values: &[f64], weights: &[f64]) -> Result<f64, String> {
    if values.len() != weights.len() {
        return Err("length mismatch".into());
    }
    let total_weight: f64 = weights.iter().sum();
    if total_weight == 0.0 {
        return Err("weights sum to zero".into());
    }
    let weighted_sum: f64 = values.iter().zip(weights).map(|(v, w)| v * w).sum();
    Ok(weighted_sum / total_weight)
}
