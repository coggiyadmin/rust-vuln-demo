// Prompt Injection FN (OWASP LLM01) — multi-hop translation relay. MISS.
#![allow(dead_code)]

pub fn relay(user_text: &str) -> String {
    let decoded = chat("Translate to English.", user_text);       // hop 1
    chat(&format!("Do exactly this:\n{}", decoded), "")           // SINK (LLM01 relay)
}

fn chat(_s: &str, _u: &str) -> String { String::new() }
