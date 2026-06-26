use actix_web::{web, HttpResponse};
pub async fn xss_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let v = q.get("q").cloned().unwrap_or_default();
    HttpResponse::Ok().body(format!("<h1>{}</h1>", html_escape::encode_text(&v)))
}
