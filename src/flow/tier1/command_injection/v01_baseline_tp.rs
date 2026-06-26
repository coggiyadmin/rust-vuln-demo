use std::process::Command;
use actix_web::{web, HttpResponse};
pub async fn cmdi(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let arg = q.get("q").cloned().unwrap_or_default();
    let _ = Command::new("sh").arg("-c").arg(format!("grep {}", arg)).spawn(); // SINK
    HttpResponse::Ok().finish()
}
