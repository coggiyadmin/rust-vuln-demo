use actix_web::{web, HttpResponse};
use std::process::Command;
pub async fn v07(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let q = q.get("q").cloned().unwrap_or_default();
    if !q.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') { return HttpResponse::Ok().finish(); }
    let _ = Command::new("grep").arg(&q).arg("/var/log/app.log").spawn();
    HttpResponse::Ok().finish()
}
