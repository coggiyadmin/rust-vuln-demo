use actix_web::{web, HttpResponse};

pub async fn location_header(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let url = q.get("url").cloned().unwrap_or_default();
    HttpResponse::Found().append_header(("Location", url)).finish() // CWE-601
}
