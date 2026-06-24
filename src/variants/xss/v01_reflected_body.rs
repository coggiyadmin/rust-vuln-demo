// XSS variant: reflected HTML body.
use actix_web::{web, HttpResponse};
pub async fn reflect(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = q.get("q").cloned().unwrap_or_default(); // SOURCE
    HttpResponse::Ok().body(format!("<h1>{}</h1>", name)) // SINK CWE-79
}
