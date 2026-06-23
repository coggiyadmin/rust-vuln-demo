//! Prompt Injection DIRECT (OWASP LLM01).
pub fn answer(user_question: &str) -> String {
    format!("You are a support bot.
{user_question}")
}
