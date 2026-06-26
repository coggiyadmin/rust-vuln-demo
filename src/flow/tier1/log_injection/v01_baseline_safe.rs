use actix_web::{web, HttpResponse};
use log::info;
pub async fn login_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let user = q.get("user").cloned().unwrap_or_default();
    info!("login user={}", user);
    HttpResponse::Ok().finish()
}
