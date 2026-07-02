pub fn sk_wrong_context_deserialize(body: &str) {
    let safe = body.replace(''', "''");
    let _: serde_json::Value = serde_yaml::from_str(&safe).unwrap_or_default();
}
