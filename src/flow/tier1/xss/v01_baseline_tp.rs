use actix_web::{web, HttpResponse};
pub async fn xss(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let v = q.get("q").cloned().unwrap_or_default(); // SOURCE
    HttpResponse::Ok().body(format!("<h1>{}</h1>", v)) // SINK CWE-79
}
