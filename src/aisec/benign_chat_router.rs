// TN — benign chat router; routes by matched key, no untrusted text reaches a prompt.
#![allow(dead_code)]

pub fn route(intent: &str) -> &'static str {
    match intent {
        "billing" => "Routing to billing.",
        "support" => "Routing to support.",
        _ => "unknown",
    }
}
