use actix_web::{web, HttpResponse};
use std::process::Command;
pub async fn v06(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let q = q.get("q").cloned().unwrap_or_default();
    safe_grep(&q);
    HttpResponse::Ok().finish()
}
