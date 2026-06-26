use actix_web::{web, HttpResponse};
use std::fs;
pub async fn path_trav(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let p = q.get("p").cloned().unwrap_or_default(); // SOURCE
    let _ = fs::read_to_string(format!("/data/{}", p)); // SINK CWE-22
    HttpResponse::Ok().finish()
}
