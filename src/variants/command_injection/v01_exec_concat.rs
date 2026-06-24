use std::process::Command;
use actix_web::{web, HttpResponse};
pub async fn cmd(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let arg = q.get("q").cloned().unwrap_or_default();
    Command::new("sh").arg("-c").arg(format!("grep {}", arg)).spawn().ok(); // SINK CWE-78
    HttpResponse::Ok().body("ok")
}
