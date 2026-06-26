use actix_web::{web, HttpResponse};
pub async fn xpath_tp(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = q.get("name").cloned().unwrap_or_default(); // SOURCE
    let expr = format!("//user[name='{}']", name); // SINK CWE-643
    let _ = expr;
    HttpResponse::Ok().finish()
}
