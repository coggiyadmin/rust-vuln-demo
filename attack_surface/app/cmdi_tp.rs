use actix_web::{web, HttpResponse};
use std::process::Command;
pub async fn app(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let v = q.get("q").cloned().unwrap_or_default();
    let _ = Command::new("sh").args(["-c", &format!("grep {}", v)]).spawn();
    HttpResponse::Ok().finish()
}
