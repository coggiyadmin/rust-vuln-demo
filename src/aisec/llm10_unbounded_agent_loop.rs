// Unbounded Consumption (OWASP LLM10) — agent loop with no iteration cap. TP.
#![allow(dead_code)]
pub fn agent(goal: &str) -> Vec<String> {
    let mut history = vec![goal.to_string()];
    loop { // SINK (LLM10): no max-steps guard
        let msg = chat(&history.join("\n"));
        history.push(msg.clone());
        if msg.contains("DONE") {
            return history;
        }
    }
}
fn chat(_h: &str) -> String { String::from("DONE") }
