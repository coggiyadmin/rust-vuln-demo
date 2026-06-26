use actix_web::{web, HttpResponse};
pub async fn ssrf_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let url = q.get("url").cloned().unwrap_or_default();
    if !url.contains("api.internal.example.com") { return HttpResponse::Forbidden().finish(); }
    let _ = reqwest::get(&url).await;
    HttpResponse::Ok().finish()
}
