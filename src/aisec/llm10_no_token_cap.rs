// Unbounded Consumption (OWASP LLM10) — no length/token cap. TP.
#![allow(dead_code)]
pub fn summarize(user_text: &str) -> String { chat(user_text) } // SINK (LLM10)
fn chat(_t: &str) -> String { String::new() }
