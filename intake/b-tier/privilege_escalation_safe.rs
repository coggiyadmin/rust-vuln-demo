use actix_web::{web, HttpResponse};

/// Safe mirror — PAT-PRIV-01
pub async fn privilege_escalation_safe(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    if req.get("staff").map(String::as_str) != Some("1") {
        return HttpResponse::Forbidden().finish();
    }
    HttpResponse::Ok()
        .insert_header(("X-Role", "admin"))
        .finish()
}
