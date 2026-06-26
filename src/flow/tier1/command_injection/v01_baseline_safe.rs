use std::process::Command;
use actix_web::{web, HttpResponse};
pub async fn cmdi_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let arg = q.get("q").cloned().unwrap_or_default();
    let _ = Command::new("grep").arg(arg).arg("/var/log/app.log").spawn();
    HttpResponse::Ok().finish()
}
