//! Tier-1 SSRF (CWE-918) — Rust per-type probe.
use actix_web::{web, HttpResponse};
pub async fn fetch(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let url = q.get("url").cloned().unwrap_or_default(); // SOURCE
    let body = reqwest::get(&url).await.unwrap().text().await.unwrap(); // SINK (CWE-918)
    HttpResponse::Ok().body(body)
}
