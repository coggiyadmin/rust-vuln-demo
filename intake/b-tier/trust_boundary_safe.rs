use actix_web::{web, HttpResponse};

const ALLOWED: &[&str] = &["user", "viewer"];

/// Safe mirror — PAT-TRUST-01
pub async fn trust_boundary_safe(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let role = req.get("role").map(String::as_str).unwrap_or("user");
    let safe = if ALLOWED.contains(&role) { role } else { "user" };
    HttpResponse::Ok().body(safe.to_string())
}
