// SAFE mirror (OWASP LLM05) — model output parsed as an integer, never executed.
#![allow(dead_code)]
pub fn run_generated(model_output: &str) -> Result<i64, std::num::ParseIntError> {
    model_output.trim().parse::<i64>()
}
