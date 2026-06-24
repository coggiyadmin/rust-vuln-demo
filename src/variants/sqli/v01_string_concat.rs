use actix_web::{web, HttpResponse};
pub async fn q(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let id = q.get("id").cloned().unwrap_or_default();
    let sql = format!("SELECT * FROM u WHERE id={}", id); // SINK CWE-89
    HttpResponse::Ok().body(sql)
}
