use actix_web::{web, HttpResponse};
use std::process::Command;
pub async fn prp_prp06(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let t = q.get("q").cloned().unwrap_or_default(); // PRP prp06
    let u = t.clone();
    let v = u;
    let _ = Command::new("sh").args(["-c", &format!("grep {}", v)]).spawn();
    HttpResponse::Ok().finish()
}
