use actix_web::{web, HttpResponse};
use std::path::PathBuf;
pub async fn path_trav_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let p = q.get("p").cloned().unwrap_or_default();
    let full = PathBuf::from("/data").join(p).canonicalize().unwrap();
    if !full.starts_with("/data") { return HttpResponse::Forbidden().finish(); }
    HttpResponse::Ok().finish()
}
