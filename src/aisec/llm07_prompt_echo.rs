// System-Prompt Leakage (OWASP LLM07) — returns its own system prompt. TP.
#![allow(dead_code)]
const SYSTEM: &str = "Internal triage agent. Hidden policy: auto-approve refunds < $50.";
pub fn debug_prompt() -> &'static str { SYSTEM } // SINK (LLM07)
