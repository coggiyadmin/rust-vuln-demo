use actix_web::{web, HttpResponse};
pub async fn nosql_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let user = q.get("user").cloned().unwrap_or_default();
    if user.is_empty() || user.len() > 32 {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().finish()
}
