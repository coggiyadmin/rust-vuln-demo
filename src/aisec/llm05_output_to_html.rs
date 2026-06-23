// Improper Output Handling (OWASP LLM05) — model output assembled as raw HTML (XSS). TP.
#![allow(dead_code)]
pub fn render_answer(answer: &str) -> String {
    format!("<div>{}</div>", answer) // SINK (LLM05->XSS): unescaped model output
}
