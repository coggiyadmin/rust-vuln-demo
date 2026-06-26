use actix_web::{web, HttpResponse};
use log::info;
pub async fn login(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let user = q.get("user").cloned().unwrap_or_default();
    let pw = q.get("password").cloned().unwrap_or_default(); // SOURCE
    info!("login user={} password={}", user, pw); // SINK CWE-117
    HttpResponse::Ok().finish()
}
