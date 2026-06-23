// Excessive Agency FN (OWASP LLM06) — confused deputy: model-chosen URL fetched with an
// ambient privileged token, no host allowlist (SSRF via tool). Expected: trust layer MISS.
#![allow(dead_code)]

pub fn fetch_tool(url: &str) -> Result<String, String> {
    let token = std::env::var("SERVICE_TOKEN").unwrap_or_default();
    // SINK (LLM06 confused-deputy): model-chosen url + ambient privileged token.
    let _req = format!("GET {} Authorization: Bearer {}", url, token);
    Ok(String::new())
}
