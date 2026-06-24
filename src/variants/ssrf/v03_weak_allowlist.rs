use actix_web::{web, HttpResponse};
pub async fn weak(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let url = q.get("url").cloned().unwrap_or_default();
    if url.contains("trusted") {
        let body = reqwest::get(&url).await.unwrap().text().await.unwrap_or_default(); // SINK CWE-918
        return HttpResponse::Ok().body(body);
    }
    HttpResponse::Ok().body("")
}
