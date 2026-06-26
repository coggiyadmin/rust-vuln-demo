use actix_web::{web, HttpResponse};
pub async fn greet(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let pat = q.get("pat").cloned().unwrap_or_else(|| "{}".into()); // SOURCE
    HttpResponse::Ok().body(format!("{}", pat)) // SINK CWE-134
}
