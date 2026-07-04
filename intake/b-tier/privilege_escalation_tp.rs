use actix_web::{web, HttpResponse};

/// B-tier PAT-PRIV-01 — query flag grants admin role (CWE-269).
pub async fn privilege_escalation_tp(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    if req.get("admin").map(String::as_str) == Some("1") {
        return HttpResponse::Ok()
            .insert_header(("X-Role", "admin")) // SINK CWE-269
            .finish();
    }
    HttpResponse::Ok().finish()
}
