//! Tier-1 unsafe deserialization (CWE-502) — Rust per-type probe.
use actix_web::{web, HttpResponse};
pub async fn restore(body: String) -> HttpResponse {
    // SINK (CWE-502): deserialize attacker-controlled data with a type-tagged format.
    let v: serde_json::Value = serde_yaml::from_str(&body).unwrap_or_default(); // SOURCE+SINK
    HttpResponse::Ok().body(v.to_string())
}
