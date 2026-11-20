use actix_web::{web, HttpResponse};

const ALLOWED: &[&str] = &["user", "viewer"];

/// Safe mirror — PAT-TRUST-01
pub async fn trust_boundary_safe(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let role = req.get("role").map(String::as_str).unwrap_or("user");
    if !ALLOWED.contains(&role) {
        return HttpResponse::BadRequest().body("bad role");
    }
    HttpResponse::Ok().body("ok")
}
