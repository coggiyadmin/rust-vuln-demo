use actix_web::{web, HttpResponse};
pub async fn sqli(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = q.get("name").cloned().unwrap_or_default(); // SOURCE
    let sql = format!("SELECT * FROM users WHERE name='{}'", name); // SINK CWE-89
    let _ = sql;
    HttpResponse::Ok().finish()
}
