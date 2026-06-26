use actix_web::{web, HttpResponse};
pub async fn crlf_tp(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let loc = q.get("url").cloned().unwrap_or_default(); // SOURCE
    HttpResponse::Ok().insert_header(("Location", loc)).finish() // SINK CWE-93
}
