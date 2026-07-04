use actix_web::{web, HttpResponse};

/// Safe mirror — PAT-INFO-01
pub async fn info_disclosure_safe(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let q = req.get("q").cloned().unwrap_or_default();
    HttpResponse::Ok().body(format!(r#"{{"status":"ok","q":"{q}"}}"#))
}
