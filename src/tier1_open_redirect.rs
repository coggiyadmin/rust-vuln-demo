//! Tier-1 open redirect (CWE-601) — Rust per-type probe.
use actix_web::{web, HttpResponse};
pub async fn go(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let next = q.get("next").cloned().unwrap_or_default(); // SOURCE
    HttpResponse::Found().append_header(("Location", next)).finish() // SINK (CWE-601)
}
