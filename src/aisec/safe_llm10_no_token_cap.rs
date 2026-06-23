// SAFE mirror (OWASP LLM10) — input length capped before the call.
#![allow(dead_code)]
const MAX_INPUT: usize = 8000;
pub fn summarize(user_text: &str) -> Result<String, String> {
    if user_text.len() > MAX_INPUT { return Err("input too large".into()); }
    Ok(chat(user_text))
}
fn chat(_t: &str) -> String { String::new() }
