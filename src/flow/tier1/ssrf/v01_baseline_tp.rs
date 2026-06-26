use actix_web::{web, HttpResponse};
pub async fn ssrf(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let url = q.get("url").cloned().unwrap_or_default(); // SOURCE
    let _ = reqwest::get(&url).await; // SINK CWE-918
    HttpResponse::Ok().finish()
}
