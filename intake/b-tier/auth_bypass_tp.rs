use actix_web::{web, HttpResponse};
use std::process::Command;

/// B-tier PAT-AUTH-01 — destructive admin action without auth (CWE-306).
pub async fn auth_bypass_tp(_req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let _ = Command::new("sh").args(["-c", "rm -rf /var/app/users"]).spawn(); // SINK CWE-306
    HttpResponse::Ok().body("purged")
}
