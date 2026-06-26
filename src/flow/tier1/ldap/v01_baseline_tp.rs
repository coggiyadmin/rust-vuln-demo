use actix_web::{web, HttpResponse};
pub async fn ldap_tp(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let uid = q.get("uid").cloned().unwrap_or_default(); // SOURCE
    let filter = format!("(uid={})", uid); // SINK CWE-90
    let _ = filter;
    HttpResponse::Ok().finish()
}
