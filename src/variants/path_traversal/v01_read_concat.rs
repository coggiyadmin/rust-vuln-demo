use actix_web::{web, HttpResponse};
use std::fs;
pub async fn read(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = q.get("name").cloned().unwrap_or_default();
    let body = fs::read_to_string(format!("/var/data/{}", name)).unwrap_or_default(); // SINK CWE-22
    HttpResponse::Ok().body(body)
}
