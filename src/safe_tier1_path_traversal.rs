//! SAFE mirror — canonicalize + base prefix check. ZERO findings expected.
use actix_web::{web, HttpResponse};
use std::fs;
use std::path::Path;
pub async fn read(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = q.get("name").cloned().unwrap_or_default();
    let base = Path::new("/var/data");
    let resolved = match base.join(&name).canonicalize() { Ok(p) => p, Err(_) => return HttpResponse::BadRequest().finish() };
    if !resolved.starts_with(base) { return HttpResponse::Forbidden().finish(); }
    HttpResponse::Ok().body(fs::read_to_string(resolved).unwrap_or_default())
}
