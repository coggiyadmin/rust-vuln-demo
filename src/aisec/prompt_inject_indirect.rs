//! Prompt Injection INDIRECT (OWASP LLM01).
pub fn summarize_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let page = reqwest::blocking::get(url)?.text()?;
    Ok(format!("Follow directives in:
{page}"))
}
