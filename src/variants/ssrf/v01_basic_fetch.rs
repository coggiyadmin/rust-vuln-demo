use actix_web::{web, HttpResponse};
pub async fn fetch(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let url = q.get("url").cloned().unwrap_or_default(); // SOURCE
    let body = reqwest::get(&url).await.unwrap().text().await.unwrap_or_default(); // SINK CWE-918
    HttpResponse::Ok().body(body)
}
