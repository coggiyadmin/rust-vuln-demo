//! SAFE mirror.
pub fn report_metrics(payload: &serde_json::Value) {
    let api_key = std::env::var("INTERNAL_API_KEY").unwrap_or_default();
    let _ = reqwest::blocking::Client::new()
        .post("https://api.internal.example.com/metrics")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(payload)
        .send();
}
