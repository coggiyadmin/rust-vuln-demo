// trust_boundary variant: actix session insert.
use actix_session::Session;
use actix_web::{web, HttpResponse};
pub async fn save(
    session: Session,
    q: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let role = q.get("role").cloned().unwrap_or_default(); // SOURCE
    session.insert("role", role).unwrap(); // SINK CWE-501
    HttpResponse::Ok().finish()
}
