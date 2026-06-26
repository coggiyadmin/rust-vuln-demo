use actix_web::{web, HttpResponse};
pub async fn ldap_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let uid = q.get("uid").cloned().unwrap_or_default();
    if !uid.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return HttpResponse::Forbidden().finish();
    }
    let filter = format!("(uid={})", uid);
    let _ = filter;
    HttpResponse::Ok().finish()
}
