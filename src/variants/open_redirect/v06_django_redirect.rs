use actix_web::{web, HttpResponse};
pub async fn actix_redirect(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let nxt = q.get("next").cloned().unwrap_or_default();
    HttpResponse::Found().append_header(("Location", nxt)).finish() // SINK CWE-601 actix redirect
}
