use actix_web::{web, HttpResponse};
pub async fn ssti(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let n = q.get("n").cloned().unwrap_or_default();
    let body = format!("<p>{}</p>", n); // template concat SSTI pattern
    HttpResponse::Ok().content_type("text/html").body(body) // SINK CWE-1336
}
