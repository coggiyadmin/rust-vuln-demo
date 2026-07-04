use actix_web::{web, HttpResponse};
use std::env;

/// B-tier PAT-INFO-01 — debug response leaks env (CWE-200).
pub async fn info_disclosure_tp(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let token = env::var("API_TOKEN").unwrap_or_default();
    let q = req.get("q").cloned().unwrap_or_default();
    HttpResponse::Ok().body(format!(r#"{{"token":"{token}","q":"{q}"}}"#)) // SINK CWE-200
}
