// Unbounded Consumption FN (OWASP LLM10) — recursive tool re-invokes the model. MISS.
#![allow(dead_code)]
pub fn expand(topic: &str) -> String {
    let sub = chat(topic);
    for line in sub.split('\n') {
        if !line.trim().is_empty() { expand(line.trim()); } // SINK (LLM10 recursive)
    }
    sub
}
fn chat(_t: &str) -> String { String::new() }
