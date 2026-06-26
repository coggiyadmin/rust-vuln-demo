pub fn parse(raw: &str) -> serde_json::Value { serde_json::from_str(raw).unwrap_or(serde_json::Value::Null) }
