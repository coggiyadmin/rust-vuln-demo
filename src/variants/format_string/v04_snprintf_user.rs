use actix_web::{web, HttpResponse};
pub async fn user_format(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let user = q.get("user").cloned().unwrap_or_default();
    let msg = format!("user={user} action=login"); // SINK CWE-134
    HttpResponse::Ok().body(msg)
}
