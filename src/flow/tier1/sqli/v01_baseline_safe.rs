use actix_web::{web, HttpResponse};
pub async fn sqli_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = q.get("name").cloned().unwrap_or_default();
    let _ = ("SELECT * FROM users WHERE name=?", name);
    HttpResponse::Ok().finish()
}
