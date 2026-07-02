use actix_web::{web, HttpResponse};
use std::process::Command;
pub async fn v05(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let q = q.get("q").cloned().unwrap_or_default();
    let _ = Command::new("grep").arg(&q).arg("/var/log/app.log").spawn();
    HttpResponse::Ok().finish()
}
