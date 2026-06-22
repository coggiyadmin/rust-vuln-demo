//! SAFE mirror — host allowlist before fetch. ZERO findings expected.
use actix_web::{web, HttpResponse};
pub async fn fetch(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let target = q.get("url").cloned().unwrap_or_default();
    let allowed = ["https://api.example.com/", "https://cdn.example.com/"];
    if !allowed.iter().any(|p| target.starts_with(p)) {
        return HttpResponse::Forbidden().finish();
    }
    let _ = reqwest::get(&target).await;
    HttpResponse::Ok().finish()
}
