// SAFE mirror (OWASP LLM10) — bounded by a hard max-steps budget.
#![allow(dead_code)]
const MAX_STEPS: usize = 8;
pub fn agent(goal: &str) -> Vec<String> {
    let mut history = vec![goal.to_string()];
    for _ in 0..MAX_STEPS {
        let msg = chat(&history.join("\n"));
        history.push(msg.clone());
        if msg.contains("DONE") {
            break;
        }
    }
    history
}
fn chat(_h: &str) -> String { String::from("DONE") }
