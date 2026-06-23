// SAFE mirror (OWASP LLM07) — no secret in the prompt; key stays server-side.
#![allow(dead_code)]
pub fn build_system() -> String {
    "You are billing-bot. Use the authorized billing tool for balances.".to_string()
}
