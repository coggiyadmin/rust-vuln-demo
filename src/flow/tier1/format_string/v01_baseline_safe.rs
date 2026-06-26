use actix_web::{web, HttpResponse};
pub async fn greet_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = q.get("name").cloned().unwrap_or_else(|| "guest".into());
    HttpResponse::Ok().body(format!("Hello {}", name))
}
