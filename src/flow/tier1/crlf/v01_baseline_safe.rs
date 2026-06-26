use actix_web::{web, HttpResponse};
pub async fn crlf_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let loc = q.get("url").cloned().unwrap_or_default();
    if loc.contains('
') || loc.contains('') {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().insert_header(("Location", loc)).finish()
}
