// Prompt Injection TOOL-OUTPUT (OWASP LLM01) — tool result given instruction authority.
#![allow(dead_code)]

// SINK (LLM01 tool-output): attacker-influenceable tool_result spliced into the system role.
pub fn run_with_tool(user_q: &str, tool_result: &str) -> String {
    let system = format!("You are an assistant. Tool said:\n{}\nNow act on it.", tool_result);
    chat_system(&system, user_q)
}

fn chat_system(_system: &str, _user: &str) -> String { String::new() }
