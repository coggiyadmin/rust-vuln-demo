use actix_web::{web, HttpResponse};
use std::process::Command;
pub async fn san_tp(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let t = q.get("q").cloned().unwrap_or_default();
    let _ = Command::new("sh").args(["-c", &format!("grep {} /var/log/app.log", t)]).spawn(); // san01 TP
    HttpResponse::Ok().finish()
}
