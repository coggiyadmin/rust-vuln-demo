// IL-64 — JSON env blob → log format (IL-5 config frontier).
pub fn audit(user_blob: &str) {
    let node: serde_json::Value = serde_json::from_str(user_blob).unwrap_or_default();
    let user = node.get("user").and_then(|v| v.as_str()).unwrap_or("");
    let pw = node.get("pw").and_then(|v| v.as_str()).unwrap_or("");
    let _ = format!("login user={} password={}", user, pw);
}
