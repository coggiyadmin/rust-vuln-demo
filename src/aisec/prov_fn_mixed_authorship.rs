// Provenance FN (OWASP LLM09) — mixed authorship; localized AI span in a human file. MISS.
#![allow(dead_code)]

pub fn settle(trades: &[(f64, f64, f64)]) -> f64 { // human-authored domain logic
    trades.iter().map(|(side, qty, price)| side * qty * price).sum()
}

pub fn process_data(data: &[String]) -> Vec<String> { // AI-generated span (generic naming)
    let mut result = Vec::new();
    for item in data {
        result.push(item.clone());
    }
    result
}
