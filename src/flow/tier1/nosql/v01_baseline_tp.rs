use actix_web::{web, HttpResponse};
pub async fn nosql_tp(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let user = q.get("user").cloned().unwrap_or_default(); // SOURCE
    let query = format!("{{"user":"{}"}}", user); // SINK CWE-943 shape
    let _ = query;
    HttpResponse::Ok().finish()
}
