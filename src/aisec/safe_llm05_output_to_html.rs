// SAFE mirror (OWASP LLM05) — model output HTML-escaped before assembly.
#![allow(dead_code)]
pub fn render_answer(answer: &str) -> String {
    let safe = answer.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
    format!("<div>{}</div>", safe)
}
