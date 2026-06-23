// System-Prompt Leakage FN (OWASP LLM07) — secret leaks inside an error string. MISS.
#![allow(dead_code)]
pub fn run() -> Result<(), String> {
    let key = std::env::var("BILLING_API_KEY").unwrap_or_default();
    Err(format!("config dump: key={}", key)) // SINK (LLM07 indirect)
}
