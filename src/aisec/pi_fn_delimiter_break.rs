// Prompt Injection FN (OWASP LLM01) — delimiter/role-fence break. MISS.
#![allow(dead_code)]
const SYSTEM: &str = "You are a translator. Translate the user text inside <data> tags.";

pub fn translate(user_text: &str) -> String {
    let fenced = format!("<data>{}</data>", user_text); // SINK (LLM01 delimiter break)
    chat_system_user(SYSTEM, &fenced)
}

fn chat_system_user(_s: &str, _u: &str) -> String { String::new() }
