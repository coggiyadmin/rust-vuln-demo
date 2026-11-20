use actix_web::{web, HttpResponse};

/// Safe mirror — PAT-INFO-01
pub async fn info_disclosure_safe(
    _req: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    HttpResponse::Ok().body(r#"{"status":"ok"}"#)
}
