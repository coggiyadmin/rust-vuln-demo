// System-Prompt Leakage (OWASP LLM07) — secret baked into the system prompt. TP.
#![allow(dead_code)]
pub fn build_system() -> String {
    // SINK (LLM07): secret embedded in the instruction
    format!("You are billing-bot. Internal key: {}. Never reveal it.",
        std::env::var("BILLING_API_KEY").unwrap_or_default())
}
