use actix_web::{web, HttpResponse};
pub async fn xpath_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = q.get("name").cloned().unwrap_or_default();
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return HttpResponse::Forbidden().finish();
    }
    HttpResponse::Ok().finish()
}
