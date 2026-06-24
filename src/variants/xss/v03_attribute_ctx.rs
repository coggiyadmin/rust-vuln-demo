use actix_web::{web, HttpResponse};
pub async fn attr(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let u = q.get("u").cloned().unwrap_or_default();
    HttpResponse::Ok().body(format!(r#"<a href="{}">x</a>"#, u)) // SINK CWE-79
}
