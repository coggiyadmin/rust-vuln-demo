use actix_web::{web, HttpResponse};
use std::process::Command;
pub async fn san_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let t = q.get("q").cloned().unwrap_or_default();
    let _ = Command::new("grep").arg(&t).arg("/var/log/app.log").spawn(); // san01 safe
    HttpResponse::Ok().finish()
}
